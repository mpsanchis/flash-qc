# Caddy Setup for Flash QC

This project now includes Caddy as a reverse proxy with automatic HTTPS.

## Quick Start

### Local Development (HTTP only)

The current configuration serves the app at `http://localhost`:

```bash
podman compose up -d
```

Access the app at: `http://localhost`

### Production Deployment (with your domain)

1. **Update the Caddyfile**:
   Edit `Caddyfile` and replace the commented section with your domain:

   ```caddy
   yourdomain.com {
       reverse_proxy nginx:80
   }
   ```

2. **Point your domain to the server**:
   - Add an A record pointing to your server's IP address
   - Wait for DNS propagation (usually 5-60 minutes)

3. **Deploy**:

   ```bash
   podman compose up -d
   ```

Caddy will automatically:

- Obtain SSL certificates from Let's Encrypt
- Renew certificates before they expire
- Redirect HTTP to HTTPS
- Enable HTTP/2 and HTTP/3

## Architecture

```text
Internet (443/80)
      ↓
   Caddy (reverse proxy + HTTPS)
      ↓
   Nginx (internal routing)
      ↓
   ├── Frontend (Astro)
   └── Backend (Rocket)
```

## Ports

- **80**: HTTP (Caddy redirects to HTTPS in production)
- **443**: HTTPS (Caddy with automatic SSL)
- **5432**: PostgreSQL (exposed for database tools)

## Volumes

- `caddy_data`: Stores SSL certificates and ACME data
- `caddy_config`: Stores Caddy configuration cache
- `postgres_data`: PostgreSQL database

## Switching Between Local and Production

### Local Development

```caddy
:80 {
    reverse_proxy nginx:80
}
```

### Production

```caddy
yourdomain.com {
    reverse_proxy nginx:80
}
```

## Troubleshooting

### Check Caddy logs

```bash
podman logs flash-qc-caddy-1
```

### Check if certificates are obtained

```bash
podman exec flash-qc-caddy-1 caddy list-certificates
```

### Test configuration

```bash
podman exec flash-qc-caddy-1 caddy validate --config /etc/caddy/Caddyfile
```

## Security Features (Production)

When you uncomment the production configuration, you get:

- ✅ Automatic HTTPS with Let's Encrypt
- ✅ HSTS (HTTP Strict Transport Security)
- ✅ Clickjacking protection
- ✅ MIME type sniffing protection
- ✅ XSS protection headers
- ✅ Gzip compression
- ✅ JSON access logs

## Notes

- Caddy requires ports 80 and 443 to obtain SSL certificates
- Make sure your firewall allows inbound traffic on these ports
- Let's Encrypt has rate limits (50 certificates per domain per week)
- For staging/testing, you can use Caddy's staging CA to avoid rate limits
