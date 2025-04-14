# Testing the gRPC + REST Demo

This document provides instructions for testing the microservice demo.

## Prerequisites

Make sure you have the following installed:
- Docker
- Docker Compose
- curl or Postman (for testing the REST API)

## Running the Services

1. From the root directory, start all services with Docker Compose:

```bash
docker-compose up --build
```

2. Wait for both services to start up. You should see log messages indicating that both the gRPC server and REST API are running.

## Testing the REST API

### Getting a Random Number

Request:
```bash
curl "http://localhost:8080/random/number?min=1&max=100"
```

Expected Response:
```json
{
  "number": 42
}
```

You can customize the range by changing the `min` and `max` query parameters.

### Getting a Random String

Request:
```bash
curl "http://localhost:8080/random/string?length=10&char_type=0"
```

Expected Response:
```json
{
  "value": "a1B2c3D4e5"
}
```

You can customize the `length` and `char_type` parameters:
- `char_type=0`: Alphanumeric (default)
- `char_type=1`: Alphabetic only
- `char_type=2`: Numeric only
- `char_type=3`: Special characters only

## Verifying the Logs

While the services are running, check the Docker Compose logs to verify that:

1. The gRPC server is logging incoming requests
2. The REST gateway is communicating with the gRPC service
3. Responses are correctly being returned

## Stopping the Services

To stop the services, press `Ctrl+C` in the terminal where Docker Compose is running, or run:

```bash
docker-compose down
```

## Troubleshooting

If the REST API returns a 503 Service Unavailable error, it might be because:
1. The gRPC service is still starting up
2. There's a network issue between the containers

Check the logs with:

```bash
docker-compose logs
```

To see logs for a specific service:

```bash
docker-compose logs random-generator
docker-compose logs rest-gateway
``` 