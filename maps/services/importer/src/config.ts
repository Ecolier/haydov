import { configDotenv } from "dotenv";
import { cleanEnv, host, port, str, url } from "envalid";

configDotenv();

const env = cleanEnv(process.env, {
  GEOGRAPHY_STORAGE_BASE_URL: url(),
  GEOGRAPHY_STORAGE_USERNAME: str(),
  GEOGRAPHY_STORAGE_PASSWORD: str(),
  GEOGRAPHY_STORAGE_REGION: str(),
  GEOGRAPHY_RAW_BUCKET_NAME: str(),
  MESSAGE_BROKER_SCHEMA: str(),
  MESSAGE_BROKER_HOST: host(),
  MESSAGE_BROKER_PORT: port(),
  MESSAGE_BROKER_USERNAME: str(),
  MESSAGE_BROKER_PASSWORD: str(),
  OPENSTREETMAP_IMPORTER_HOST: host(),
  OPENSTREETMAP_IMPORTER_PORT: port(),
});

export default env;