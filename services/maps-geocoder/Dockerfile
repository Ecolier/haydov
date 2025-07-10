FROM pelias/openstreetmap:master AS pelias

FROM node:current AS deps

WORKDIR /app

RUN corepack enable && corepack prepare pnpm@latest --activate

# Copy and install dependencies
COPY --from=workspace_root pnpm-lock.yaml ./
COPY --from=workspace_root package.json ./
COPY --from=workspace_root nx.json ./

RUN pnpm install --frozen-lockfile