ARG rust_manifests=rust-manifests
ARG utils_sources=utils-sources
ARG data_provider=data-provider
ARG workspace=maps/services/downloader

FROM ${rust_manifests} AS manifests
FROM ${utils_sources} AS utils-sources
FROM ${data_provider} AS data-provider

FROM rust:1.88 AS chef

RUN cargo install cargo-chef

WORKDIR /app

COPY --from=manifests / ./
COPY --from=utils-sources / ./

FROM chef AS planner

ARG workspace

COPY ./src ./${workspace}/src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ARG workspace

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . ./${workspace}

RUN cargo build --release

FROM debian:bookworm-slim AS prod

WORKDIR /app

COPY --from=builder /app/target/release/maps-downloader /usr/local/bin

CMD ["tail", "-f", "/dev/null"]