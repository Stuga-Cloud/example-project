FROM rust:1.70 as builder

WORKDIR /app

ARG DATABASE_URL

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

# Install OpenSSL
RUN apt-get update \
    && apt-get install -y openssl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/

WORKDIR /usr/local/bin

EXPOSE 8080

CMD ["./backend"]
