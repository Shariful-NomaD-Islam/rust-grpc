use tonic::{transport::Server, Request, Response, Status};
use rand::Rng;

pub mod random {
    tonic::include_proto!("random");
}

use random::random_service_server::{RandomService, RandomServiceServer};
use random::{RandomRequest, RandomReply};

#[derive(Default)]
pub struct MyRandomService;

#[tonic::async_trait]
impl RandomService for MyRandomService {
    async fn get_random(
        &self,
        _request: Request<RandomRequest>,
    ) -> Result<Response<RandomReply>, Status> {
        let random_value = rand::thread_rng().gen_range(1000..9999).to_string();
        Ok(Response::new(RandomReply { value: random_value }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyRandomService::default();

    println!("Worker gRPC server listening on {}", addr);

    Server::builder()
        .add_service(RandomServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
