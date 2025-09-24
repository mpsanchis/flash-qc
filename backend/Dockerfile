FROM rust:1.89 AS builder

WORKDIR /app
COPY Cargo.toml ./
COPY Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/flashqc-backend /app/flashqc-backend
COPY Rocket.toml /app/Rocket.toml

EXPOSE 8000

CMD ["./flashqc-backend"]