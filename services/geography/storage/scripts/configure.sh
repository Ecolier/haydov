#!/bin/bash

mc alias set local http://localhost:$MINIO_API_PORT "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD"

# Create bucket if not exists
mc ls local/$BUCKET_NAME || mc mb local/$BUCKET_NAME

# Loop over each webhook entry
mc admin info --json local | jq -r .info.sqsARN[] | while read -r arn; do
  [ -n "$(mc event ls local/$BUCKET_NAME "$arn")" ] || mc event add local/$BUCKET_NAME "$arn" --event put
done