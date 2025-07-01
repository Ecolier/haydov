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

# Add crontab file in the cron directory
ADD crontab /etc/cron.d/hello-cron

# Give execution rights on the cron job
RUN chmod 0644 /etc/cron.d/hello-cron

# Create the log file to be able to run tail
RUN touch /var/log/cron.log

#Install Cron
RUN apt-get update
RUN apt-get -y install cron


# Run the command on container startup
CMD cron && tail -f /var/log/cron.log

EXPOSE 3000
CMD ["maps"]