#!/bin/bash

set -e

wait_until_ready() {
  until curl -s http://localhost:$MINIO_CONSOLE_PORT/minio/health/ready; do
    sleep 1
  done
}

wait_until_ready

# Setup alias for mc
mc alias set local http://localhost:$MINIO_API_PORT "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD"

# Create bucket if not exists
mc ls local/$OSM_BUCKET_NAME || mc mb local/$OSM_BUCKET_NAME

# Loop over each webhook entry
jq -r '.webhooks[].endpoint' ./config.json | while read -r endpoint; do
  mc admin config set local notify_webhook:osmhook \
    endpoint="$endpoint" \
    queue_limit="0"
done

# Save and apply the config
mc admin service restart local --json

wait_until_ready

# Enable webhook for bucket
mc event add local/$OSM_BUCKET_NAME arn:minio:sqs::osmhook:webhook --event put