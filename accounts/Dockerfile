ARG NODE_ENV

FROM node:18-alpine AS base

FROM base AS deps
RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY package.json yarn.lock* ./
COPY .yarn ./.yarn
COPY accounts/package.json ./accounts/
RUN yarn install --immutable 

FROM base AS builder-production
WORKDIR /app
COPY accounts ./accounts
COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/.yarn ./.yarn
COPY --from=deps /app/yarn.lock /app/package.json ./

ENV NEXT_TELEMETRY_DISABLED 1

RUN yarn workspace @haydov/accounts build

FROM base AS finalizer-production
WORKDIR /app
ENV NODE_ENV production
ENV NEXT_TELEMETRY_DISABLED 1
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
COPY --from=builder-production /app/accounts/public ./accounts/public
COPY --from=builder-production --chown=nextjs:nodejs /app/accounts/.next/standalone ./
COPY --from=builder-production --chown=nextjs:nodejs /app/accounts/.next/static ./accounts/.next/static
USER nextjs
EXPOSE 3000
ENV PORT 3000
CMD ["node", "./accounts/server.js"]

FROM base AS finalizer-development
WORKDIR /app
ENV NODE_ENV development
ENV NEXT_TELEMETRY_DISABLED 1
COPY --from=deps /app ./
COPY accounts ./accounts
EXPOSE 3000
ENV PORT 3000
CMD ["yarn", "workspace", "@haydov/accounts", "dev"]

FROM finalizer-${NODE_ENV} as runner