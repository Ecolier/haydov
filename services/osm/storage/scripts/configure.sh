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
mc admin info --json local | jq .info.sqsARN[] | while read -r arn; do 
  mc event ls local/$OSM_BUCKET_NAME "$arn" || mc event add local/$OSM_BUCKET_NAME "$arn"
done