FROM pelias/openstreetmap:master AS pelias-openstreetmap
FROM pelias/whosonfirst:master AS pelias-whosonfirst
FROM pelias/openaddresses:master AS pelias-openaddresses

FROM node:current AS deps

WORKDIR /app

RUN corepack enable && corepack prepare pnpm@latest --activate

# Copy and install dependencies
COPY --from=workspace_root pnpm-lock.yaml ./
COPY --from=workspace_root package.json ./
COPY --from=workspace_root pnpm-workspace.yaml ./
COPY --from=workspace_root nx.json ./

COPY ./package.json ./services/geocoding/importer/

RUN pnpm install --frozen-lockfile

# Copy the Pelias codebase
COPY --from=pelias-openstreetmap /code /code

# Copy the geocoding-importer service code
COPY . ./services/geocoding/importer/

# Build the application
RUN pnpm nx build geocoding-importer

EXPOSE 4000
ENTRYPOINT [ "pnpm" ]
CMD [ "nx", "dev", "geocoding-importer" ]

# FROM node:current AS builder

# WORKDIR /app

# RUN corepack enable && corepack prepare pnpm@latest --activate

# COPY --from=deps /app .

# RUN pnpm nx build my-app