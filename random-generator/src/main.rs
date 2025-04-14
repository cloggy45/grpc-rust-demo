use anyhow::Result;
use dotenv::dotenv;
use log::{info, error};
use rand::{thread_rng, Rng};
use std::env;
use tonic::{transport::Server, Request, Response, Status};

// Import the generated proto code
pub mod random {
    tonic::include_proto!("random");
}

// Import the generated service trait and message types
use random::random_generator_server::{RandomGenerator, RandomGeneratorServer};
use random::{NumberRequest, NumberResponse, StringRequest, StringResponse};

// Implement the RandomGenerator service
#[derive(Debug, Default)]
pub struct RandomGeneratorService {}

#[tonic::async_trait]
impl RandomGenerator for RandomGeneratorService {
    async fn get_random_number(
        &self,
        request: Request<NumberRequest>,
    ) -> Result<Response<NumberResponse>, Status> {
        let req = request.into_inner();
        info!("Received request for random number between {} and {}", req.min, req.max);
        
        // Validate input
        if req.min > req.max {
            return Err(Status::invalid_argument("min must be less than or equal to max"));
        }
        
        // Generate random number
        let number = thread_rng().gen_range(req.min..=req.max);
        info!("Generated random number: {}", number);
        
        // Return response
        Ok(Response::new(NumberResponse { number }))
    }

    async fn get_random_string(
        &self,
        request: Request<StringRequest>,
    ) -> Result<Response<StringResponse>, Status> {
        let req = request.into_inner();
        info!("Received request for random string of length {}", req.length);
        
        // Validate input
        if req.length <= 0 {
            return Err(Status::invalid_argument("length must be greater than 0"));
        }
        
        // Determine character set based on char_type
        let charset = match req.char_type {
            0 => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            1 => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
            2 => "0123456789",
            3 => "!@#$%^&*()_+-=[]{}|;:,.<>?",
            _ => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", // Default to alphanumeric
        };
        
        // Generate random string
        let mut rng = thread_rng();
        let value: String = (0..req.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();
        
        info!("Generated random string: {}", value);
        
        // Return response
        Ok(Response::new(StringResponse { value }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Get server address from environment or use default
    let host = env::var("GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("{}:{}", host, port).parse()?;
    
    info!("Random Generator gRPC server starting on {}", addr);
    
    // Create and start the gRPC server
    let service = RandomGeneratorService::default();
    
    Server::builder()
        .add_service(RandomGeneratorServer::new(service))
        .serve(addr)
        .await?;
    
    Ok(())
} 