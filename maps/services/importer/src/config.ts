import { configDotenv } from "dotenv";
import { cleanEnv, host, port, str, url } from "envalid";

configDotenv();

const env = cleanEnv(process.env, {
  MAPS_STORAGE_BASE_URL: url(),
  MAPS_STORAGE_USERNAME: str(),
  MAPS_STORAGE_PASSWORD: str(),
  MAPS_STORAGE_REGION: str(),
  MAPS_RAW_BUCKET_NAME: str(),
  MESSAGE_BROKER_SCHEMA: str(),
  MESSAGE_BROKER_HOST: host(),
  MESSAGE_BROKER_PORT: port(),
  MESSAGE_BROKER_USERNAME: str(),
  MESSAGE_BROKER_PASSWORD: str(),
  OPENSTREETMAP_IMPORTER_HOST: host(),
  OPENSTREETMAP_IMPORTER_PORT: port(),
});

export default env;