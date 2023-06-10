FROM rust:1.70 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/backend /usr/local/bin/

WORKDIR /usr/local/bin

EXPOSE 8080

CMD ["./backend"]
