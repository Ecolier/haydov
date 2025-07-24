FROM scratch AS pnpm-root

COPY pnpm-lock.yaml /
COPY package.json /
COPY pnpm-workspace.yaml /
COPY nx.json /

FROM scratch AS cargo-root

COPY Cargo.toml /
COPY Cargo.lock /