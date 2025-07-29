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
COPY --from=data-provider /app/world.wit \
  /app/target/wasm32-wasip2/release/data_provider.wasm \
  /data-provider/

CMD ["tail", "-f", "/dev/null"]
RUN cargo build --release

FROM rust:1.88 AS bacon-cache

WORKDIR /root
ENV CARGO_HOME=/root/.cargo
RUN cargo install --locked bacon

FROM chef AS dev 

COPY --from=bacon-cache /root/.cargo /root/.cargo

ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_HOME=/root/.cargo

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .

EXPOSE 3000
CMD ["bacon", "--headless"]

FROM debian:bookworm-slim AS prod

WORKDIR /app

COPY --from=builder /app/target/release/app /usr/local/bin

EXPOSE 3000
CMD ["geography-dispatcher"]