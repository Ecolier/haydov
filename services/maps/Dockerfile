FROM rust:1.87 AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/maps /usr/local/bin/maps
CMD ["maps"]