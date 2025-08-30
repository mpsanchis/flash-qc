# Docker Compose Multi-Service Setup

This project demonstrates a simple multi-service architecture using Docker Compose with:

- **Nginx**: Reverse proxy (port 80)
- **Astro Frontend**: Dummy frontend application (internal port 3000)
- **Rocket Backend**: Rust API server (internal port 8000)
- **PostgreSQL**: Database (internal port 5432)

## Quick Start

1. Build and start all services:

   ```bash
   docker-compose up -d --build
   ```

2. Access the application:
   - Frontend: <http://localhost>
   - Backend API: <http://localhost/api/hello>

3. Stop all services:

   ```bash
   docker-compose down
   ```

## Architecture

- The nginx reverse proxy routes:
  - `/` → Frontend (Astro)
  - `/api/` → Backend (Rocket)
- All services communicate through a shared Docker network
- PostgreSQL data persists in a Docker volume

## Development

- Frontend code is in `./frontend/`
- Backend code is in `./backend/`
- Nginx configuration is in `./nginx/nginx.conf`

### Docker Compose Build Notes

**When you need to build:**

- First time running `docker-compose up`
- After changing Dockerfile or build context files
- After adding new dependencies (package.json, Cargo.toml, etc.)
- After modifying source code that gets copied into the image

**When you don't need to build:**

- Just restarting existing containers
- Only changed environment variables or volumes
- Only using pre-built images (like PostgreSQL)

**Useful commands:**

```bash
# Uses existing images, builds if none exist
docker-compose up

# Forces rebuild of all services
docker-compose up --build

# Just builds without starting
docker-compose build

# Rebuilds specific service
docker-compose up --build <service>
```

### Database Access

PostgreSQL is configured with `ports` instead of `expose` for development convenience:

- **`expose`**: Makes port available only to other containers within the Docker network
- **`ports`**: Maps container port to host port, allows external access from localhost

This allows you to connect database GUI tools, run migrations from your host, or debug directly at `localhost:5432`.
