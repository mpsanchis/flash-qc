FROM node:18-alpine

RUN npm install -g pnpm

WORKDIR /app

# Copy workspace package files
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./

# Copy frontend package.json
COPY frontend/package.json ./frontend/

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy application files
COPY frontend ./frontend

WORKDIR /app/frontend

EXPOSE 3000

CMD ["pnpm", "run", "dev:production"]