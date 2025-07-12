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

FROM chef AS development 

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .

RUN cargo install --locked bacon

EXPOSE 3000
CMD ["bacon"]

FROM debian:bookworm-slim AS production

WORKDIR /app

COPY --from=builder /app/target/release/app /usr/local/bin

ADD crontab /etc/cron.d/hello-cron
RUN chmod 0644 /etc/cron.d/hello-cron
RUN touch /var/log/cron.log
RUN apt-get update
RUN apt-get -y install cron

EXPOSE 3000

CMD ["cron", "&&", "tail", "-f", "/var/log/cron.log"]