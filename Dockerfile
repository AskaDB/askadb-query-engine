FROM rust:1.74-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy main.rs and copy real source code
RUN rm src/main.rs
COPY src/ src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Create app user
RUN addgroup -g 1001 -S app && \
    adduser -S app -u 1001

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/askadb-query-engine /app/askadb-query-engine

# Create data directory
RUN mkdir -p /app/data && chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 8002

# Start the application
CMD ["./askadb-query-engine"]
