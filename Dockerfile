FROM scratch AS pnpm-workspace

COPY pnpm-lock.yaml /
COPY package.json /
COPY pnpm-workspace.yaml /
COPY nx.json /

FROM scratch AS cargo-workspace

COPY Cargo.toml /
COPY Cargo.lock /