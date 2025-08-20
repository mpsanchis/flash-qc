FROM rust:1.81 as builder

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

COPY --from=builder /app/target/release/dummy-backend /app/dummy-backend

EXPOSE 8000

CMD ["./dummy-backend"]