FROM rust:1.84 AS builder

ARG SERVICE_NAME=service_b

WORKDIR /app

RUN --mount=type=bind,source=services,target=services \
  --mount=type=bind,source=common,target=common \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=cache,target=/app/target \
  --mount=type=cache,target=/usr/local/cargo/registry \
  cargo build --locked --bin ${SERVICE_NAME} && cp /app/target/release/${SERVICE_NAME} /app/bin

FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /app/bin /usr/local/bin/service

ENV RUST_LOG=info

CMD service
