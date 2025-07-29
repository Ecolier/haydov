# syntax=docker/dockerfile:1.7-labs

FROM scratch AS rust-manifests
ARG workspace
COPY --exclude=**/*.rs ./${workspace} /${workspace}

FROM scratch AS rust-sources
ARG workspace
COPY --exclude=**/Cargo.toml ./${workspace} /${workspace}