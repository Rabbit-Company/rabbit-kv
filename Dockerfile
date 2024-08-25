FROM rust:1.80-alpine as builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release


FROM alpine:3.20

COPY --from=builder /app/target/release/rabbit-kv /usr/local/bin/rabbit-kv

CMD ["rabbit-kv"]