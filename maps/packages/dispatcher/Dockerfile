ARG workspace_root=workspace-root

FROM rust:1.88 AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin app

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