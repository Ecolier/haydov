{
  # Infrastructure endpoints (local services)
  infrastructure = {
    MAPS_STORAGE_BASE_URL = "localhost:9000";
    COMMON_MESSAGE_BROKER_BASE_URL = "amqp://guest:guest@localhost:5672";
  };

  # Runtime configuration
  runtime = {
    LOG_LEVEL = "debug";
    ENVIRONMENT = "local";
    RUST_LOG = "debug";
  };

  # Secrets for local development
  secrets = {
    MAPS_STORAGE_USERNAME = "admin";
    MAPS_STORAGE_PASSWORD = "password";
    COMMON_MESSAGE_BROKER_USERNAME = "admin";
    COMMON_MESSAGE_BROKER_PASSWORD = "password";
  };
}