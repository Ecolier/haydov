#!/bin/bash

until mc alias set local $GEOGRAPHY_STORAGE_BASE_URL "$GEOGRAPHY_STORAGE_USERNAME" "$GEOGRAPHY_STORAGE_PASSWORD"; do
  echo "Waiting for MinIO..."
  sleep 2
done

mc ls local/$GEOGRAPHY_RAW_BUCKET_NAME || mc mb local/$GEOGRAPHY_RAW_BUCKET_NAME
mc ls local/$GEOGRAPHY_PROCESSED_BUCKET_NAME || mc mb local/$GEOGRAPHY_PROCESSED_BUCKET_NAME

mc admin info --json local | jq -r '(.info.sqsARN // [])[]' | while read -r arn; do
  [ -n "$(mc event ls local/$GEOGRAPHY_RAW_BUCKET_NAME "$arn")" ] || mc event add local/$GEOGRAPHY_RAW_BUCKET_NAME "$arn" --event put
done

tail -f /dev/null