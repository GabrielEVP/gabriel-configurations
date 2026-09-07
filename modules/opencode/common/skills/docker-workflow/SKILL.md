---
name: docker-workflow
description: Docker-first development workflow. Always use docker-compose exec, never run locally. Container rebuild without data loss, test execution in containers.
license: MIT
compatibility: opencode
---

# Docker-First Development Workflow

## Core Rule

**NEVER run commands directly on the host machine.** All development commands go through Docker. This ensures every developer has the same environment.

---

## Standard Commands

### Spring Boot Backend

```bash
# Start all services
docker-compose up -d

# Stop all services
docker-compose down

# View logs (follow)
docker-compose logs -f app

# View logs for specific service
docker-compose logs -f db

# Run tests (ALL)
docker-compose exec app ./mvnw test

# Run specific test class
docker-compose exec app ./mvnw test -Dtest=OrderUseCaseTest

# Run tests in a specific package
docker-compose exec app ./mvnw test -Dtest="com.company.app.orders.*"

# Build application (skip tests)
docker-compose exec app ./mvnw clean package -DskipTests

# Build application (with tests)
docker-compose exec app ./mvnw clean package

# Apply Flyway migrations manually
docker-compose exec app ./mvnw flyway:migrate

# Flyway info (see migration status)
docker-compose exec app ./mvnw flyway:info

# Check application health
curl http://localhost:8080/actuator/health
```

### Angular Frontend

```bash
# Start frontend dev server (if separate container)
docker-compose up -d frontend

# Run tests
docker-compose exec frontend npm test

# Run tests with coverage
docker-compose exec frontend npm run test:coverage

# Build for production
docker-compose exec frontend npm run build

# Install new dependency
docker-compose exec frontend npm install primeng

# Run linting
docker-compose exec frontend npm run lint
```

### Database

```bash
# Connect to PostgreSQL
docker-compose exec db psql -U postgres -d appdb

# Run a SQL query
docker-compose exec db psql -U postgres -d appdb -c "SELECT * FROM customers LIMIT 10;"

# Create a database dump
docker-compose exec db pg_dump -U postgres appdb > backup.sql

# Restore from dump
docker-compose exec -T db psql -U postgres -d appdb < backup.sql

# View database size
docker-compose exec db psql -U postgres -d appdb -c "\l+"
```

---

## Rebuild Without Data Loss

```bash
# Rebuild app container only (DB data preserved in volume)
docker-compose up -d --build app

# Rebuild frontend only
docker-compose up -d --build frontend

# Rebuild all containers (DB data preserved — volume survives)
docker-compose up -d --build

# DESTRUCTIVE: Remove containers AND volumes (loses all data)
# Only use this to completely reset development environment
docker-compose down -v
```

---

## Common docker-compose.yaml Structure

```yaml
version: '3.8'

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile.dev
    ports:
      - "8080:8080"
    environment:
      - DB_URL=jdbc:postgresql://db:5432/appdb
      - DB_USER=postgres
      - DB_PASSWORD=postgres
      - JWT_SECRET=${JWT_SECRET}
    volumes:
      - .:/workspace
      - ~/.m2:/root/.m2  # Cache Maven dependencies
    depends_on:
      db:
        condition: service_healthy

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.dev
    ports:
      - "4200:4200"
    volumes:
      - ./frontend:/app
      - /app/node_modules  # Don't mount host node_modules
    environment:
      - API_URL=http://localhost:8080

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: appdb
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    volumes:
      - postgres_data:/var/lib/postgresql/data  # Named volume = data persists
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:  # Persists even when containers are removed
```

---

## Development Dockerfile (Hot Reload)

```dockerfile
# Dockerfile.dev — Spring Boot with hot reload
FROM eclipse-temurin:24-jdk

WORKDIR /workspace

COPY .mvn .mvn
COPY mvnw .

RUN ./mvnw dependency:go-offline -q

COPY pom.xml .
COPY src src

CMD ["./mvnw", "spring-boot:run", "-Dspring-boot.run.jvmArguments=-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:5005"]
```

---

## Test Container Setup

```bash
# Run tests with test database (Testcontainers handles this automatically)
docker-compose exec app ./mvnw test

# Run only integration tests
docker-compose exec app ./mvnw test -Dtest="**/*IntegrationTest"

# Run only unit tests (no Spring context)
docker-compose exec app ./mvnw test -Dtest="**/*Test" -pl "!**/integration"
```

---

## Environment Variables

```bash
# .env.example — commit this
DB_URL=jdbc:postgresql://db:5432/appdb
DB_USER=postgres
DB_PASSWORD=postgres
JWT_SECRET=change-me-in-production

# .env — gitignored, actual values
JWT_SECRET=very-long-random-secret-key-here
```

---

## Debugging

```bash
# Check container status
docker-compose ps

# Inspect a container
docker inspect parsun-app-1

# Execute shell in running container
docker-compose exec app bash

# View resource usage
docker stats

# Check container logs for errors
docker-compose logs app | grep -i error

# Rebuild specific service after dependency change
docker-compose up -d --build --no-deps app
```

---

## Rules

1. **Never** run `./mvnw` or `npm` commands directly on the host
2. **Always** use `docker-compose exec {service} {command}`
3. **Never** use `docker-compose down -v` unless you want to lose all data
4. **Always** use named volumes for database data
5. **Always** cache Maven and npm dependencies in volumes for faster builds
6. **Never** hardcode secrets — use environment variables from `.env`
