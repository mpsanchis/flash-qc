FROM rust:1.91-trixie AS backend-builder

WORKDIR /app
COPY backend/Cargo.toml ./
COPY backend/Cargo.lock* ./
COPY backend/src ./src

RUN cargo build --release

# Build plugins
FROM rust:1.91-trixie AS plugin-builder

# Install Node.js and npm
RUN apt-get update && \
    apt-get install -y nodejs npm && \
    rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN cargo install wasm-pack

WORKDIR /

# Copy TypeScript config from root
COPY tsconfig.base.json ./

WORKDIR /plugins

# Copy all plugins
COPY plugins ./

# Build TypeScript/JavaScript plugins
RUN if [ -f flip-word/package.json ]; then \
      cd flip-word && npm install && npm run build; \
    fi

# Build WASM plugins
RUN if [ -f drawing-canvas/Cargo.toml ]; then \
      cd drawing-canvas && wasm-pack build --target web; \
    fi

# Final runtime image
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=backend-builder /app/target/release/flashqc-backend /app/flashqc-backend
COPY backend/Rocket.toml /app/Rocket.toml
COPY --from=plugin-builder /plugins /app/plugins

EXPOSE 8000

CMD ["./flashqc-backend"]