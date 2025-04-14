# gRPC + REST Microservice Demo in Rust

This project demonstrates a microservice architecture in Rust, showing how to combine gRPC for internal service communication and REST for external APIs.

## Architecture

The application consists of two microservices:

1. **Random Generator Service** (`random-generator/`):
   - A gRPC-based microservice that generates random data
   - Provides two RPC methods:
     - `GetRandomNumber`: Generates a random number between min and max values
     - `GetRandomString`: Generates a random string with specified length and character set

2. **REST Gateway** (`rest-gateway/`):
   - An HTTP REST API that internally calls the Random Generator gRPC service
   - Exposes two endpoints:
     - `GET /random/number?min=X&max=Y`: Get a random number
     - `GET /random/string?length=X&char_type=Y`: Get a random string

## Technologies Used

- **Rust**: The programming language for both services
- **Tonic**: gRPC framework for Rust
- **Axum**: Lightweight web framework for the REST API
- **Protobuf**: For defining the service interface
- **Docker & Docker Compose**: For containerization and orchestration
- **Tokio**: Async runtime for Rust

## Project Structure

```
grpc-rest-demo/
├── proto/                   # Protobuf definitions
│   └── random.proto        # gRPC service definition
├── random-generator/       # gRPC service implementation
│   ├── src/
│   │   └── main.rs         # gRPC server implementation
│   ├── build.rs            # Build script for protobuf compilation
│   ├── Cargo.toml          # Dependencies and package metadata
│   └── Dockerfile          # Docker configuration
├── rest-gateway/           # REST API service
│   ├── src/
│   │   └── main.rs         # REST API implementation
│   ├── build.rs            # Build script for protobuf compilation
│   ├── Cargo.toml          # Dependencies and package metadata
│   └── Dockerfile          # Docker configuration
├── Cargo.toml              # Workspace-level dependencies
├── docker-compose.yml      # Service orchestration
├── .dockerignore           # Files to exclude from Docker builds
├── .env                    # Environment variables
└── README.md               # Project documentation
```

## Running the Demo

### Prerequisites

- Docker and Docker Compose
- Rust (if you want to run services directly)

### Using Docker Compose (Recommended)

1. Build and start all services:

```bash
docker-compose up --build
```

2. Access the REST API:
   - Random Number: `http://localhost:8080/random/number?min=1&max=100`
   - Random String: `http://localhost:8080/random/string?length=10&char_type=0`

3. Stop the services:

```bash
docker-compose down
```

### Running Locally (Development)

1. Start the Random Generator service:

```bash
cd random-generator
cargo run
```

2. In another terminal, start the REST Gateway:

```bash
cd rest-gateway
cargo run
```

3. Access the REST API at `http://localhost:8080/`

## Configuration

Both services use environment variables for configuration:

### Random Generator Service
- `GRPC_HOST`: Host for the gRPC server (default: "0.0.0.0")
- `GRPC_PORT`: Port for the gRPC server (default: "50051")
- `RUST_LOG`: Log level (default: "info")

### REST Gateway Service
- `REST_HOST`: Host for the REST server (default: "0.0.0.0")
- `REST_PORT`: Port for the REST server (default: "8080")
- `GENERATOR_HOST`: Host of the Random Generator service (default: "random-generator" in Docker, "127.0.0.1" otherwise)
- `GENERATOR_PORT`: Port of the Random Generator service (default: "50051")
- `RUST_LOG`: Log level (default: "info")

## API Documentation

### REST API

#### Get a Random Number
- Endpoint: `GET /random/number`
- Query Parameters:
  - `min` (optional): Minimum value (default: 1)
  - `max` (optional): Maximum value (default: 100)
- Response: `{ "number": 42 }`

#### Get a Random String
- Endpoint: `GET /random/string`
- Query Parameters:
  - `length` (optional): Length of the string (default: 10)
  - `char_type` (optional): Type of characters to include:
    - `0`: Alphanumeric (default)
    - `1`: Alphabetic only
    - `2`: Numeric only
    - `3`: Special characters only
- Response: `{ "value": "a1B2c3D4e5" }`

## Next Steps / Improvements

Some potential enhancements for this demo:
- Add authentication and authorization
- Implement metrics collection
- Add health checks and circuit breakers
- Implement automated testing
- Set up CI/CD pipelines 