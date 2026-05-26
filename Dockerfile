# Multi-stage Dockerfile for Paladin
# Optimized for production deployment with minimal image size
# Supports multi-architecture builds (amd64, arm64)

# =============================================================================
# Stage 1: Builder
# Builds the application with all dependencies
# Note: Using Bookworm (Debian 12) for OpenSSL 3.x support
# =============================================================================
FROM rust:1.93-slim-bookworm AS builder
WORKDIR /app

# Install required build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Copy all source files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY crates ./crates
COPY migrations ./migrations
# config.yml is gitignored (env-specific); provide at runtime via volume mount

# Build the application in release mode
# --workspace ensures all crates are resolved correctly
RUN cargo build --release --workspace --bin paladin

# Strip debug symbols to reduce binary size
RUN strip target/release/paladin

# =============================================================================
# Stage 2: Runtime
# Minimal runtime image with only the binary
# =============================================================================
FROM debian:12-slim
WORKDIR /app

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/paladin /usr/local/bin/paladin

# Copy migrations (config.yml must be provided at runtime via volume mount)
COPY --from=builder /app/migrations /app/migrations

# Create non-root user
RUN groupadd -g 65532 paladin && \
    useradd -u 65532 -g paladin -s /bin/false -M paladin && \
    chown -R paladin:paladin /app

# Use non-root user
USER paladin:paladin

# Expose ports
EXPOSE 8080 9090

# Health check (distroless has limited shell, so we use simple approach)
# Note: Kubernetes liveness/readiness probes will handle health checking
HEALTHCHECK NONE

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/paladin"]

# Default command (can be overridden)
CMD ["--help"]
