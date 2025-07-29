FROM rust:1.88 AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner

COPY . ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . ./

RUN rustup target add wasm32-wasip2
RUN cargo build --release --target wasm32-wasip2