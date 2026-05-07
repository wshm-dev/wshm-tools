# syntax=docker/dockerfile:1

# ────────────────────────────────────────────────────────────────
# Stage 1 — builder
# ────────────────────────────────────────────────────────────────
ARG ALPINE_VERSION=3.23
FROM rust:1-alpine$ALPINE_VERSION AS builder

COPY --from=oven/bun:1-alpine --chmod=a=rX /usr/local/bin/bun /usr/local/bin/

WORKDIR /build
COPY . .
RUN cd web && bun install --frozen-lockfile && bun run build
RUN apk update && apk add --no-cache musl-dev perl make ca-certificates
RUN cargo build --release --bin wshm && strip target/release/wshm

# ────────────────────────────────────────────────────────────────
# Stage 2 — runtime
# ────────────────────────────────────────────────────────────────
FROM alpine:$ALPINE_VERSION

RUN apk update && apk add --no-cache -X https://dl-cdn.alpinelinux.org/alpine/$${ALPINE_VERSION}/community ca-certificates libgit2  \
    && adduser -S wshm -h /home/wshm -s /bin/false

COPY --from=builder /build/target/release/wshm /usr/local/bin/wshm

USER wshm
WORKDIR /home/wshm

ENV WSHM_HOME=/home/wshm/.wshm
EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/wshm"]
CMD ["daemon"]
