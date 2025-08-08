import env from "./config.js";

import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';

import { GetObjectCommand, S3Client } from "@aws-sdk/client-s3";
import { S3Event } from "aws-lambda";
import { connect } from "amqplib";

import path from "path";

import { mkdir, readFile, writeFile } from "fs/promises";
import { createWriteStream } from "fs";
import { pipeline } from "stream/promises";

const client = new S3Client({
  endpoint: env.MAPS_STORAGE_BASE_URL,
  region: env.MAPS_STORAGE_REGION,
  forcePathStyle: true, // Required for MinIO or local S3-compatible services
  credentials: {
    accessKeyId: env.MAPS_STORAGE_USERNAME,
    secretAccessKey: env.MAPS_STORAGE_PASSWORD,
  },
});

// Load protobuf definition
const packageDefinition = protoLoader.loadSync('./openstreetmap/.proto', {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

const importService = grpc.loadPackageDefinition(packageDefinition).ImportService as any;

const grpcClient = new importService(
  `${env.OPENSTREETMAP_IMPORTER_HOST}:${env.OPENSTREETMAP_IMPORTER_PORT}`,
  grpc.credentials.createInsecure()
);

const exchange = "haydov.maps";
const queue = "dispatch";

(async () => {
  try {
    const connection = await connect(
      `${env.MESSAGE_BROKER_SCHEMA}://${env.MESSAGE_BROKER_USERNAME}:${env.MESSAGE_BROKER_PASSWORD}@${env.MESSAGE_BROKER_HOST}:${env.MESSAGE_BROKER_PORT}`
    );
    process.once("SIGINT", async () => {
      await connection.close();
    });
    
    const channel = await connection.createChannel();
    await channel.assertExchange(exchange, "fanout", { durable: false });
    await channel.assertQueue(queue, { durable: true });
    await channel.bindQueue(queue, exchange, "");
    await channel.prefetch(1);
    
    const importFiles: string[] = [];
    
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
          const config = JSON.parse(
            (await readFile("/code/pelias.json")).toLocaleString()
          );
          config.imports.openstreetmap.import = importFiles.map((file) => ({
            filename: file,
          }));
          await writeFile("/code/pelias.json", JSON.stringify(config, null, 2));
        }
        
        const objectBasename = path.basename(objectKey);
        
        await pipeline(
          output.Body as NodeJS.ReadableStream,
          createWriteStream(path.join("/data", objectBasename))
        );
        
        importFiles.push(objectBasename);
        
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
