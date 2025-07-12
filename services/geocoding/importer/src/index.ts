import fastify from "fastify";
import { GetObjectCommand, S3Client } from "@aws-sdk/client-s3";
import { configDotenv } from "dotenv";
import { cleanEnv, str } from "envalid";
import { S3Event } from "aws-lambda";
import { connect } from "amqplib";

import {createWriteStream} from "fs";

configDotenv();

// Validate environment variables
const env = cleanEnv(process.env, {
  AWS_S3_ENDPOINT: str({ default: "http://osm-storage:9000/" }),
  AWS_ACCESS_KEY_ID: str({ default: "haydov_access" }),
  AWS_SECRET_ACCESS_KEY: str({ default: "haydov_secret" }),
  AWS_REGION: str({ default: "us-west-2" }),
  AWS_DEFAULT_REGION: str({ default: "us-west-2" }),
  OSM_BUCKET_NAME: str({ default: "osm-dumps" }),
  MESSAGE_BROKER_URL: str({ default: "amqp://haydov_test_user:haydov_test_password@message:5672/" }),
});

const client = new S3Client({
  endpoint: env.AWS_S3_ENDPOINT,
  region: env.AWS_REGION,
  forcePathStyle: true, // Required for MinIO or local S3-compatible services
  credentials: {
    accessKeyId: env.AWS_ACCESS_KEY_ID,
    secretAccessKey: env.AWS_SECRET_ACCESS_KEY,
  }
});

connect(env.MESSAGE_BROKER_URL).then((connection) => {
  console.log("Connected to message broker");
  connection.createChannel().then((channel) => {
    channel.assertExchange("haydov.osm", "fanout", { durable: false });
    channel.assertQueue("dispatch", { durable: true });
    channel.bindQueue("dispatch", "haydov.osm", "");
    channel.consume("dispatch", (msg) => {
      if (msg !== null) {
        const event = JSON.parse(msg.content.toString());
        console.log("Received message:", event);
        // Process the message as needed
        channel.ack(msg);
      }
    });
  });
}).catch((error) => {
  console.error("Error connecting to message broker:", error);
});

const app = fastify();
app.post("/update-osm", async (request, reply) => {
  const req = request.body as unknown as S3Event;

  if (!req || !req.Records || req.Records.length === 0) {
    return reply.status(400).send({ error: "Invalid S3 event data." });
  }

  const record = req.Records[0];
  const bucketName = record.s3.bucket.name;
  const objectKey = record.s3.object.key;

  const response = await client.send(new GetObjectCommand({
    Bucket: bucketName,
    Key: objectKey,
  }));

  const stream = response.Body as NodeJS.ReadableStream;
  const writable = createWriteStream(`/data/${objectKey}`);
  stream.pipe(writable);
  
  writable.on("finish", () => {
    console.log(`File ${objectKey} downloaded successfully.`);
    reply.send({ message: `File ${objectKey} downloaded successfully.` });
  });

  writable.on("error", (err) => {
    console.error(`Error writing file ${objectKey}:`, err);
    reply.status(500).send({ error: "Failed to write file." });
  });

});

app.listen({ host: "0.0.0.0", port: 4000 }, (err, address) => {
  if (err) {
    console.error(err);
    process.exit(1);
  }
  console.log(`Server listening on ${address}`);
});
