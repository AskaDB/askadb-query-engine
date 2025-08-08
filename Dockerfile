FROM rust:1.74-alpine

# Install build dependencies
RUN apk add --no-cache musl-dev libgcc

# Set working directory
WORKDIR /app

# Copy sources
COPY Cargo.toml ./
COPY src/ src/

# Build the application (release)
RUN cargo build --release

# Data directory
RUN mkdir -p /app/data

# Expose port
EXPOSE 8002

# Env for logging
ENV RUST_LOG=info

# Run the binary
CMD ["/app/target/release/askadb-query-engine"]
