import { GetObjectCommand, S3Client } from "@aws-sdk/client-s3";
import { configDotenv } from "dotenv";
import { cleanEnv, str } from "envalid";
import { S3Event } from "aws-lambda";
import { connect } from "amqplib";

import path from "path";

import { mkdir } from "fs/promises";
import { createWriteStream } from "fs";
import { pipeline } from "stream/promises";

configDotenv();

// Validate environment variables
const env = cleanEnv(process.env, {
  AWS_S3_ENDPOINT: str({ default: "http://osm-storage:9000/" }),
  AWS_ACCESS_KEY_ID: str({ default: "haydov_access" }),
  AWS_SECRET_ACCESS_KEY: str({ default: "haydov_secret" }),
  AWS_REGION: str({ default: "us-west-2" }),
  AWS_DEFAULT_REGION: str({ default: "us-west-2" }),
  OSM_BUCKET_NAME: str({ default: "osm-dumps" }),
  MESSAGE_BROKER_URL: str({
    default: "amqp://haydov_test_user:haydov_test_password@message:5672/",
  }),
});

const client = new S3Client({
  endpoint: env.AWS_S3_ENDPOINT,
  region: env.AWS_REGION,
  forcePathStyle: true, // Required for MinIO or local S3-compatible services
  credentials: {
    accessKeyId: env.AWS_ACCESS_KEY_ID,
    secretAccessKey: env.AWS_SECRET_ACCESS_KEY,
  },
});

const exchange = "haydov.osm";
const queue = "dispatch";

(async () => {
  try {
    const connection = await connect(env.MESSAGE_BROKER_URL);
    process.once("SIGINT", async () => {
      await connection.close();
    });

    const channel = await connection.createChannel();
    await channel.assertExchange(exchange, "fanout", { durable: false });
    await channel.assertQueue(queue, { durable: true });
    await channel.bindQueue(queue, exchange, "");
    await channel.prefetch(1);

    await channel.consume(queue, async (msg) => {
      if (msg === null) return;

      try {
        const content = JSON.parse(msg.content.toString()) as S3Event;

        const bucketName = content.Records[0].s3.bucket.name;
        const objectKey = decodeURIComponent(content.Records[0].s3.object.key);

        const output = await client.send(
          new GetObjectCommand({
            Bucket: bucketName,
            Key: objectKey,
          })
        );

        if (output.ContentLength === 0) {
          console.warn(`⚠️ Skipping 0-byte sentinel file: ${objectKey}`);
          channel.ack(msg);
          return;
        }

        await mkdir(path.join("/data", path.dirname(objectKey)), {
          recursive: true,
        });

        await pipeline(
          output.Body as NodeJS.ReadableStream,
          createWriteStream(path.join("/data", objectKey))
        );

        console.log(
          `📂 Download object: ${objectKey} from bucket: ${bucketName}`
        );

        channel.ack(msg);
      } catch (err) {
        console.error(`Error processing message:`, err);
        channel.nack(msg, false, false); // reject message without requeueing
      }
    });

    console.log(`✅ Waiting for messages in ${queue}. To exit press CTRL+C`);
  } catch (err) {
    console.error("❌ Error connecting to RabbitMQ:", err);
    process.exit(1);
  }
})();
