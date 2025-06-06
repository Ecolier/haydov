FROM rust:latest AS builder
WORKDIR /usr/src/maps
COPY . .
RUN cargo install --path .

FROM builder AS development
RUN cargo install --locked bacon
EXPOSE 3000
CMD ["bacon", "webserver"]

FROM debian:latest AS production
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/maps /usr/local/bin/maps

EXPOSE 3000
CMD ["maps"]