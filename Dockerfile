FROM rust:1.86-bullseye AS builder

WORKDIR /usr/src/relay

COPY src src
COPY Cargo.lock Cargo.toml .

RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/relay /usr/local/bin/relay

ENV SECRET_KEY=0000000000000000000000000000000000000000000000000000000000000000
ENV PORT=5000

CMD ["sh", "-c", "relay --secret-key $SECRET_KEY --port $PORT"]
