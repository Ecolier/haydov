# This is the root of the workspace,
# where the pnpm workspace and nx files are located.
ARG workspace_root=workspace-root
ARG package_path=services/geography/importer
ARG package_name=geography-importer

FROM ${workspace_root} AS workspace-root

FROM node:current-slim AS pnpm-base

ARG package_path

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

COPY --from=workspace-root / /app/
COPY ./package.json ./tsconfig.json /app/${package_path}/

WORKDIR /app

FROM pnpm-base AS dev

ARG package_path

# Install dependencies
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

# Copy the geography-importer service code
COPY ./src ./${package_path}/src

EXPOSE 3000
ENTRYPOINT [ "pnpm" ]
CMD [ "nx", "dev", "geography-importer" ]

FROM pnpm-base AS build

ARG package_name

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

COPY ./src ./${package_path}/src
RUN pnpm nx build geography-importer

RUN pnpm deploy --filter=${package_name} --prod /dist

FROM pnpm-base AS prod

COPY --from=build /dist /dist
WORKDIR /dist
EXPOSE 3000
CMD [ "pnpm", "start" ]