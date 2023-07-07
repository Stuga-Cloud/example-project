FROM rust:1.70 as builder

WORKDIR /app

ARG DATABASE_URL

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y openssl libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/

WORKDIR /usr/local/bin

EXPOSE 9000

CMD ["./backend"]
