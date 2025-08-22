#!/bin/bash

set -euxo pipefail

until mc alias set local $STORAGE_BASE_URL "$STORAGE_USERNAME" "$STORAGE_PASSWORD"; do
  echo "Waiting for MinIO..."
  sleep 2
done

# Create buckets if they don't exist
for var in $(env | grep '^BUCKET_NAME_' | cut -d= -f1); do
  bucket="${!var}"
  mc ls "local/$bucket" || mc mb "local/$bucket"
done

# Iterate over all buckets for event setup
for var in $(env | grep '^BUCKET_NAME_' | cut -d= -f1); do
  bucket="${!var}"
  mc admin info --json local | jq -r '(.info.sqsARN // [])[]' | while read -r arn; do
    [ -n "$(mc event ls local/$bucket "$arn")" ] || mc event add local/$bucket "$arn" --event put
  done
done