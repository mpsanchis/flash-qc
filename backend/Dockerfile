FROM rust:1.91-trixie AS builder

WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src

RUN cargo build --release

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/flashqc-backend /app/flashqc-backend
COPY backend/Rocket.toml /app/Rocket.toml
COPY plugins /app/plugins

EXPOSE 8000

CMD ["./flashqc-backend"]