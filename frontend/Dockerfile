# Build: docker build -t frontend .
# Run: docker run frontend
FROM node:22-alpine AS builder
WORKDIR /app
COPY website /app/
ENV npm_config_loglevel=error
RUN npm ci --force
RUN npm run build

FROM danjellz/http-server:1.4 AS final
WORKDIR /public
COPY --from=builder ./app/dist .
CMD ["http-server"]
