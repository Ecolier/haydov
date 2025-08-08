#!/bin/bash

echo "$STORAGE_BASE_URL" "$STORAGE_USERNAME" "$STORAGE_PASSWORD"
until mc alias set local $STORAGE_BASE_URL "$STORAGE_USERNAME" "$STORAGE_PASSWORD"; do
  echo "Waiting for MinIO..."
  sleep 2
done

mc ls local/$RAW_BUCKET_NAME || mc mb local/$RAW_BUCKET_NAME
mc ls local/$PROCESSED_BUCKET_NAME || mc mb local/$PROCESSED_BUCKET_NAME

mc admin info --json local | jq -r '(.info.sqsARN // [])[]' | while read -r arn; do
  [ -n "$(mc event ls local/$RAW_BUCKET_NAME "$arn")" ] || mc event add local/$RAW_BUCKET_NAME "$arn" --event put
done

tail -f /dev/null