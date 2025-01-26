# https://hackmd.io/jgkoQ24YRW6i0xWd73S64A

FROM rust:1.84 AS builder

WORKDIR /app

RUN cargo new common
RUN cargo new services/service_a

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY common/Cargo.toml common/Cargo.toml
COPY services/service_a/Cargo.toml services/service_a/Cargo.toml
RUN cargo build --release --bin service_a

COPY services/service_a/src/ services/service_a/src/
COPY common/src/ common/src/
RUN cargo build --release --bin service_a

FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /app/target/release/service_a /usr/local/bin/service_a

ENV RUST_LOG=info

CMD ["service_a"]
