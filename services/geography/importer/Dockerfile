ARG workspace_root

FROM pelias/openstreetmap:master AS pelias-openstreetmap
FROM pelias/whosonfirst:master AS pelias-whosonfirst
FROM pelias/openaddresses:master AS pelias-openaddresses

FROM ${workspace_root} AS workspace-root

FROM node:current AS development

WORKDIR /app

RUN corepack enable && corepack prepare pnpm@latest --activate

# Copy and install dependencies
COPY --from=workspace-root / ./

COPY ./package.json ./services/geocoding/importer/

RUN pnpm install --frozen-lockfile

# Copy the Pelias codebase
COPY --from=pelias-openstreetmap /code /code
COPY --from=pelias-whosonfirst /code /code
COPY --from=pelias-openaddresses /code /code

# Copy the geography-importer service code
COPY . ./services/geography/importer/

# Build the application
RUN pnpm nx build geography-importer

EXPOSE 3000
ENTRYPOINT [ "pnpm" ]
CMD [ "nx", "dev", "geography-importer" ]

# FROM node:current AS builder

# WORKDIR /app

# RUN corepack enable && corepack prepare pnpm@latest --activate

# COPY --from=deps /app .

# RUN pnpm nx build my-app