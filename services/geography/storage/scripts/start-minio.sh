#!/bin/bash

export MINIO_NOTIFY_AMQP_URL_PRIMARY="amqp://$MESSAGE_BROKER_USERNAME:$MESSAGE_BROKER_PASSWORD@message-broker:5672"
minio server /bitnami/minio/data