FROM pelias/openstreetmap:master AS pelias

FROM node:current AS deps

WORKDIR /app

RUN corepack enable && corepack prepare pnpm@latest --activate

# Copy and install dependencies
COPY --from=workspace_root pnpm-lock.yaml ./
COPY --from=workspace_root package.json ./
COPY --from=workspace_root pnpm-workspace.yaml ./
COPY --from=workspace_root nx.json ./

COPY ./package.json ./services/maps-geocoder/

RUN pnpm install --frozen-lockfile

# Copy the Pelias codebase
COPY --from=pelias /code /code

# Copy the maps-geocoder service code
COPY . ./services/maps-geocoder/

# Build the application
RUN pnpm nx build maps-importer

EXPOSE 3000
ENTRYPOINT [ "pnpm" ]
CMD [ "nx", "dev", "maps-importer" ]

# FROM node:current AS builder

# WORKDIR /app

# RUN corepack enable && corepack prepare pnpm@latest --activate

# COPY --from=deps /app .

# RUN pnpm nx build my-app