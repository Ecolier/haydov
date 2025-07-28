FROM pelias/openstreetmap:latest

USER root

# Install gRPC dependencies
RUN apt-get update && apt-get install -y \
    python3-grpc-tools \
    python3-grpcio \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app && chown -R pelias:pelias /app

# Copy proto definition and gRPC server
COPY import_service.proto /app/
COPY grpc-server.py /app/
COPY entrypoint.sh /app/

RUN chmod +x /app/entrypoint.sh

USER pelias

EXPOSE 50051

ENTRYPOINT ["/app/entrypoint.sh"]