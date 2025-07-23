#!/bin/bash

until mc alias set local http://$MINIO_SERVER_HOST:9000 "$MINIO_SERVER_ACCESS_KEY" "$MINIO_SERVER_SECRET_KEY"; do
  echo "Waiting for MinIO..."
  sleep 2
done

mc ls local/$BUCKET_NAME || mc mb local/$BUCKET_NAME

mc admin info --json local | jq -r '(.info.sqsARN // [])[]' | while read -r arn; do
  [ -n "$(mc event ls local/$BUCKET_NAME "$arn")" ] || mc event add local/$BUCKET_NAME "$arn" --event put
done

tail -f /dev/null