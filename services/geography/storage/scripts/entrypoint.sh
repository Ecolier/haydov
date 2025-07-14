#!/bin/bash

set -e

export MINIO_NOTIFY_AMQP_URL_PRIMARY="amqp://$MESSAGE_BROKER_USERNAME:$MESSAGE_BROKER_PASSWORD@$MESSAGE_BROKER_ENDPOINT"
echo "Using AMQP URL: $MINIO_NOTIFY_AMQP_URL_PRIMARY"

minio server --console-address :9001 /data & MINIO_PID=$!

{
until curl -s http://localhost:$MINIO_CONSOLE_PORT/minio/health/ready; do
  sleep 1
done

./scripts/configure.sh
} &

wait $MINIO_PID

