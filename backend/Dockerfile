FROM rust:1.70 as builder

WORKDIR /app

RUN apt-get -y update && \
    apt-get -y install libssl1.0.0 libssl-dev && \
    ln -s libssl.so.1.0.0 libssl.so.10 && \
    ln -s libcrypto.so.1.0.0 libcrypto.so.10

ARG DATABASE_URL

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/backend /usr/local/bin/

WORKDIR /usr/local/bin

EXPOSE 8080

CMD ["./backend"]
