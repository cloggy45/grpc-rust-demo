FROM rust:1.75-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the rest-gateway service
RUN cargo build --release --package rest-gateway

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rest-gateway /app/rest-gateway

# Install OpenSSL, CA certificates, and netcat (required for healthcheck)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates netcat-openbsd && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set environment variables with defaults
ENV REST_HOST=0.0.0.0
ENV REST_PORT=8080
ENV GENERATOR_HOST=random-generator
ENV GENERATOR_PORT=50051

# Expose the REST port
EXPOSE 8080

# Run the service
CMD ["/app/rest-gateway"] 