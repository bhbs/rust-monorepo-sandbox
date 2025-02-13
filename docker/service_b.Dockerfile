# https://hackmd.io/jgkoQ24YRW6i0xWd73S64A

FROM rust:1.84 AS builder

ARG SERVICE_NAME=service_b

WORKDIR /app

RUN cargo new common
RUN cargo new services/${SERVICE_NAME}

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY common/Cargo.toml common/Cargo.toml
COPY services/${SERVICE_NAME}/Cargo.toml services/${SERVICE_NAME}/Cargo.toml
RUN cargo build --release --bin ${SERVICE_NAME}

COPY services/${SERVICE_NAME}/src/ services/${SERVICE_NAME}/src/
COPY common/src/ common/src/
RUN cargo build --release --bin ${SERVICE_NAME}

FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /app/target/release/${SERVICE_NAME} /usr/local/bin/service

ENV RUST_LOG=info

ENTRYPOINT ["service"]
