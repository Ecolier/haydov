FROM minio/minio:latest

RUN curl -L -o /usr/bin/jq https://github.com/jqlang/jq/releases/download/jq-1.6/jq-linux64
RUN chmod +x /usr/bin/jq

WORKDIR /app

ADD --chown=minio:minio . .
RUN chmod +x ./scripts/configure.sh

ENTRYPOINT [ "/bin/sh", "-c"]
CMD ["./scripts/configure.sh & minio server --console-address :9001 /data"]