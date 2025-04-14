use anyhow::Result;
use axum::{
    extract::Query,
    routing::get,
    Router,
    http::StatusCode,
    Json,
};
use dotenv::dotenv;
use log::{info, error};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

// Import the generated proto code
pub mod random {
    tonic::include_proto!("random");
}

// Import the generated client types
use random::random_generator_client::RandomGeneratorClient;
use random::{NumberRequest, StringRequest};

// Shared state for the application
struct AppState {
    client: Mutex<Option<RandomGeneratorClient<tonic::transport::Channel>>>,
}

// Query parameters for the random number endpoint
#[derive(Deserialize)]
struct NumberParams {
    min: Option<i32>,
    max: Option<i32>,
}

// Query parameters for the random string endpoint
#[derive(Deserialize)]
struct StringParams {
    length: Option<i32>,
    char_type: Option<i32>,
}

// Response types for the REST API
#[derive(Serialize)]
struct NumberResponse {
    number: i32,
}

#[derive(Serialize)]
struct StringResponse {
    value: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Connect to the gRPC service
async fn connect_to_grpc() -> Result<RandomGeneratorClient<tonic::transport::Channel>> {
    // Get gRPC service address from environment variables
    let host = env::var("GENERATOR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("GENERATOR_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("http://{}:{}", host, port);
    
    info!("Connecting to gRPC service at {}", addr);
    let client = RandomGeneratorClient::connect(addr).await?;
    
    Ok(client)
}

// Handler for GET /random/number
async fn get_random_number(
    Query(params): Query<NumberParams>,
    state: axum::extract::State<Arc<AppState>>,
) -> Result<Json<NumberResponse>, (StatusCode, Json<ErrorResponse>)> {
    let min = params.min.unwrap_or(1);
    let max = params.max.unwrap_or(100);
    
    info!("REST request for random number between {} and {}", min, max);
    
    // Access the mutex-protected client
    let mut client_guard = state.client.lock().await;
    
    // If client is None, try to reconnect
    if client_guard.is_none() {
        match connect_to_grpc().await {
            Ok(client) => {
                *client_guard = Some(client);
            }
            Err(e) => {
                error!("Failed to connect to gRPC service: {}", e);
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ErrorResponse {
                        error: "Failed to connect to random number service".to_string(),
                    }),
                ));
            }
        }
    }
    
    // Make the gRPC request
    match client_guard.as_mut() {
        Some(client) => {
            let request = tonic::Request::new(NumberRequest { min, max });
            
            match client.get_random_number(request).await {
                Ok(response) => {
                    let number = response.into_inner().number;
                    info!("Received random number: {}", number);
                    Ok(Json(NumberResponse { number }))
                }
                Err(status) => {
                    error!("gRPC error: {}", status);
                    
                    // Handle disconnection by clearing the client
                    if status.code() == tonic::Code::Unavailable {
                        *client_guard = None;
                    }
                    
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: format!("Random number service error: {}", status),
                        }),
                    ))
                }
            }
        }
        None => Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "Random number service not available".to_string(),
            }),
        )),
    }
}

// Handler for GET /random/string
async fn get_random_string(
    Query(params): Query<StringParams>,
    state: axum::extract::State<Arc<AppState>>,
) -> Result<Json<StringResponse>, (StatusCode, Json<ErrorResponse>)> {
    let length = params.length.unwrap_or(10);
    let char_type = params.char_type.unwrap_or(0);
    
    info!("REST request for random string of length {} with char_type {}", length, char_type);
    
    // Access the mutex-protected client
    let mut client_guard = state.client.lock().await;
    
    // If client is None, try to reconnect
    if client_guard.is_none() {
        match connect_to_grpc().await {
            Ok(client) => {
                *client_guard = Some(client);
            }
            Err(e) => {
                error!("Failed to connect to gRPC service: {}", e);
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ErrorResponse {
                        error: "Failed to connect to random string service".to_string(),
                    }),
                ));
            }
        }
    }
    
    // Make the gRPC request
    match client_guard.as_mut() {
        Some(client) => {
            let request = tonic::Request::new(StringRequest { length, char_type });
            
            match client.get_random_string(request).await {
                Ok(response) => {
                    let value = response.into_inner().value;
                    info!("Received random string: {}", value);
                    Ok(Json(StringResponse { value }))
                }
                Err(status) => {
                    error!("gRPC error: {}", status);
                    
                    // Handle disconnection by clearing the client
                    if status.code() == tonic::Code::Unavailable {
                        *client_guard = None;
                    }
                    
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: format!("Random string service error: {}", status),
                        }),
                    ))
                }
            }
        }
        None => Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "Random string service not available".to_string(),
            }),
        )),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Get REST server address from environment or use default
    let host = env::var("REST_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("REST_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
    
    info!("REST Gateway starting on {}", addr);
    
    // Initialize shared state
    let client = match connect_to_grpc().await {
        Ok(client) => {
            info!("Successfully connected to gRPC service");
            Some(client)
        }
        Err(e) => {
            error!("Initial connection to gRPC service failed: {}. Will retry on first request.", e);
            None
        }
    };
    
    let app_state = Arc::new(AppState {
        client: Mutex::new(client),
    });
    
    // Build our application with routes
    let app = Router::new()
        .route("/random/number", get(get_random_number))
        .route("/random/string", get(get_random_string))
        .with_state(app_state);
    
    // Start the server
    info!("REST API ready to serve requests");
    axum::serve(tokio::net::TcpListener::bind(&addr).await?, app).await?;
    
    Ok(())
} 