#!/bin/bash
set -e

# Generate Python gRPC code
python3 -m grpc_tools.protoc \
    --python_out=/app \
    --grpc_python_out=/app \
    --proto_path=/app \
    import_service.proto

# Start gRPC server in background
python3 /app/grpc-server.py &

# Keep container running
tail -f /dev/null