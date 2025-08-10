#!/bin/bash

set -euo pipefail

export MINIO_NOTIFY_AMQP_URL_PRIMARY="$MESSAGE_BROKER_SCHEMA://$MESSAGE_BROKER_USERNAME:$MESSAGE_BROKER_PASSWORD@$MESSAGE_BROKER_HOST:$MESSAGE_BROKER_PORT"
exec /opt/bitnami/minio/bin/minio server \
    --address :9000 \
    --console-address :9001 \
    /bitnami/minio/data