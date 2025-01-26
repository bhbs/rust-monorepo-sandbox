FROM rust:1.84 AS builder

WORKDIR /app

RUN cargo new common
RUN cargo new services/service_b

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY common/Cargo.toml common/Cargo.toml
COPY services/service_b/Cargo.toml services/service_b/Cargo.toml
RUN cargo build --release --bin service_b

COPY services/service_b/src/ services/service_b/src/
COPY common/src/ common/src/
RUN cargo build --release --bin service_b

FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /app/target/release/service_b /usr/local/bin/service_b

ENV RUST_LOG=info

CMD ["service_b"]
