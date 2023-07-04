ARG NODE_ENV

FROM quay.io/keycloak/keycloak:21.1.2 as base

FROM base as builder-development

CMD ["start-dev \
  --spi-theme-static-max-age=-1 \
  --spi-theme-cache-themes=false \
  --spi-theme-cache-templates=false"]

FROM base as builder-production

CMD ["start"]

FROM builder-${NODE_ENV} as runner