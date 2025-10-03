# FileFire Cloud API Docker Image
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY core/Cargo.toml ./core/
COPY cloud/api/Cargo.toml ./cloud/api/
COPY plugins/*/Cargo.toml ./plugins/*/

# Copy source code
COPY core/src ./core/src
COPY cloud/api/src ./cloud/api/src
COPY plugins/*/src ./plugins/*/src

# Build the application
RUN cargo build --release --bin filefire-api

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false filefire

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/filefire-api /app/
COPY --from=builder /app/target/release/deps/lib*.so /usr/local/lib/

# Create directories for uploads and temp files
RUN mkdir -p /app/uploads /app/temp && \
    chown -R filefire:filefire /app

# Set user
USER filefire

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["./filefire-api"]