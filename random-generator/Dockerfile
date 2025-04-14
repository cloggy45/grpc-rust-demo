FROM rust:1.75-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the random-generator service
RUN cargo build --release --package random-generator

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/random-generator /app/random-generator

# Install OpenSSL, CA certificates, and netcat (required for healthcheck)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates netcat-openbsd && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set environment variables with defaults
ENV GRPC_HOST=0.0.0.0
ENV GRPC_PORT=50051

# Expose the gRPC port
EXPOSE 50051

# Run the service
CMD ["/app/random-generator"] 