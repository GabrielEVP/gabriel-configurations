---
name: docker-advanced
description: "Trigger: dockerfile, docker-compose, multi-stage, healthcheck, docker network, docker volume, compose profiles, docker production, container optimization. Advanced Docker — multi-stage builds, healthchecks, networks, volumes, compose profiles, production patterns, optimization."
---

# Advanced Docker

## Multi-Stage Builds

```dockerfile
# ===== Stage 1: Build =====
FROM maven:3.9-eclipse-temurin-24 AS builder
WORKDIR /app
COPY pom.xml .
RUN mvn dependency:go-offline -q
COPY src/ src/
RUN mvn package -q -DskipTests

# ===== Stage 2: Runtime =====
FROM eclipse-temurin:24-jre AS runtime
WORKDIR /app

# Create non-root user
RUN groupadd -r app && useradd -r -g app app

# Copy only the artifact from builder stage
COPY --from=builder /app/target/*.jar app.jar

# Security: drop capabilities
RUN setcap -r /opt/java/openjdk/bin/java
USER app

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD curl -f http://localhost:8080/actuator/health || exit 1

ENTRYPOINT ["java", "-jar", "app.jar"]
```

```dockerfile
# Node.js multi-stage
FROM node:22-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine AS runtime
COPY --from=builder /app/dist/app/browser /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=5s CMD wget -qO- http://localhost/
```

## Healthchecks

```dockerfile
# Dockerfile healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Options:
# --interval=30s    # How often to run the check
# --timeout=3s      # Max time for the check to complete
# --start-period=10s # Grace period before first check
# --retries=3       # Consecutive failures before marking unhealthy
```

```yaml
# docker-compose healthcheck
services:
  app:
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/actuator/health"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 15s

  db:
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      interval: 10s
      timeout: 3s
      retries: 5

  depends_on with condition:
  app:
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
```

## Docker Compose Profiles

```yaml
services:
  app:
    image: myapp:latest
    ports: ["8080:8080"]

  db:
    image: mysql:8.0
    profiles: ["dev"]      # Only starts in dev profile

  mailpit:
    image: axllent/mailpit
    ports: ["8025:8025"]
    profiles: ["dev", "staging"]

  monitoring:
    image: prom/prometheus
    profiles: ["monitoring"]

# Usage:
# docker compose up                    # app + db (no profile = services without profiles)
# docker compose --profile dev up      # app + db + mailpit
# docker compose --profile monitoring up
# docker compose --profile dev --profile monitoring up
```

## Networks

```yaml
services:
  app:
    networks:
      - frontend
      - backend

  db:
    networks:
      - backend

  nginx:
    networks:
      - frontend

networks:
  frontend:
    driver: bridge
    ipam:
      config:
        - subnet: "172.20.0.0/16"
  backend:
    driver: bridge
    internal: true        # No external internet access
```

```bash
# List networks
docker network ls

# Inspect network
docker network inspect <network>

# Connect a running container to a network
docker network connect <network> <container>

# Disconnect a container
docker network disconnect <network> <container>

# Create network with custom driver
docker network create --driver overlay --attachable my-network
```

## Volumes

```yaml
services:
  db:
    image: mysql:8.0
    volumes:
      - mysql-data:/var/lib/mysql
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql:ro
      - ./backups:/backups

volumes:
  mysql-data:
    driver: local
    driver_opts:
      type: none
      device: /data/mysql
      o: bind
```

```bash
# List volumes
docker volume ls

# Inspect volume
docker volume inspect <volume>

# Create volume
docker volume create <name>

# Remove unused volumes
docker volume prune

# Backup a volume
docker run --rm -v <volume>:/data -v $(pwd):/backup alpine tar czf /backup/backup.tar.gz -C /data .

# Restore a volume
docker run --rm -v <volume>:/data -v $(pwd):/backup alpine tar xzf /backup/backup.tar.gz -C /data
```

## Production Patterns

```yaml
services:
  app:
    image: myapp:${TAG:-latest}
    restart: unless-stopped
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: '1'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    environment:
      - SPRING_PROFILES_ACTIVE=prod
    env_file:
      - .env.prod
```

### Resource Limits

```yaml
services:
  app:
    # Hard limits
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
    # Soft reservations
    deploy:
      resources:
        reservations:
          cpus: '0.5'
          memory: 256M

  db:
    deploy:
      resources:
        limits:
          memory: 2G
```

## Dockerfile Optimization

```dockerfile
# 1. Order layers by change frequency (least → most)
FROM node:22-alpine
WORKDIR /app
COPY package*.json ./      # rarely changes
RUN npm ci                 # cached unless package.json changes
COPY . .                   # changes most often — last

# 2. Use .dockerignore
# .dockerignore
node_modules
.git
*.md
dist
.env*

# 3. Pin versions (never use :latest)
FROM node:22-alpine              # ✅ good
FROM node:22.14.0-alpine         # ✅ best
FROM node:latest                 # ❌ never

# 4. Combine RUN commands
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 5. Use --no-cache for apk (Alpine)
FROM alpine:3.20
RUN apk add --no-cache curl

# 6. COPY --chown to avoid root
COPY --chown=app:app --from=builder /app/dist /usr/share/nginx/html
```

## Image Optimization

```bash
# Show image size
docker images
docker system df

# Show layer sizes
docker history <image>

# Scan for vulnerabilities
docker scout quick <image>
docker scout recommendations <image>

# Analyze image
dive <image>    # requires: brew install dive

# Remove dangling images
docker image prune

# Remove unused images
docker image prune -a

# Export and compress
docker save <image> | gzip > image.tar.gz
docker load < image.tar.gz

# Multi-architecture build
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t myapp:latest \
  --push .
```

## Docker Compose Production vs Development

```yaml
# docker-compose.yml (base — shared config)
services:
  app:
    image: myapp:${TAG:-latest}
    environment:
      - DB_URL=jdbc:mysql://db:3306/appdb
    depends_on:
      db:
        condition: service_healthy
  
  db:
    image: mysql:8.0
    volumes:
      - db-data:/var/lib/mysql
    environment:
      MYSQL_ROOT_PASSWORD: ${DB_ROOT_PASSWORD}
      MYSQL_DATABASE: appdb

volumes:
  db-data:
```

```yaml
# docker-compose.override.yml (dev — auto-loaded)
services:
  app:
    build: .
    ports:
      - "8080:8080"
      - "5005:5005"      # debug port
    environment:
      - SPRING_PROFILES_ACTIVE=dev
      - JAVA_TOOL_OPTIONS=-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:5005
    volumes:
      - ./src:/app/src    # hot reload

  db:
    ports:
      - "3306:3306"
  
  mailpit:
    image: axllent/mailpit
    ports: ["8025:8025"]
```

```yaml
# docker-compose.prod.yml (production)
services:
  app:
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 512M
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

  db:
    deploy:
      resources:
        limits:
          memory: 2G

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/conf.d/default.conf:ro
      - ./ssl:/etc/nginx/ssl:ro

# Usage:
# docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Debugging & Troubleshooting

```bash
# Check container logs
docker logs -f <container>

# Check resource usage
docker stats

# Execute command in running container
docker exec -it <container> bash

# Debug networking
docker exec <container> ping db
docker exec <container> nslookup db

# Check container details
docker inspect <container>

# Show processes inside container
docker top <container>

# Copy files in/out
docker cp <container>:/app/logs ./
docker cp ./config.yml <container>:/app/config.yml

# Debug build cache
docker builder prune
docker build --no-cache .

# Check why container exited
docker logs <container>
docker inspect <container> --format='{{.State.ExitCode}}'
docker inspect <container> --format='{{.State.Error}}'
```

## Security Best Practices

```dockerfile
# 1. Run as non-root
RUN groupadd -r app && useradd -r -g app app
USER app

# 2. Drop capabilities
RUN setcap -r /path/to/binary
# Or in compose:
# cap_drop: [ALL]

# 3. Read-only root filesystem
# In compose: read_only: true

# 4. No new privileges
# In compose: security_opt: [no-new-privileges:true]

# 5. Use tmpfs for /tmp
# In compose: tmpfs: [/tmp]

# 6. Don't run SSH in containers
# 7. Don't store secrets in images (use secrets or env files)
# 8. Scan images: docker scout quick <image>
# 9. Pin base image versions
# 10. Use distroless or alpine when possible
```
