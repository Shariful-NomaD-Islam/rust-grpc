use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::transport::Channel;
use random::random_service_client::RandomServiceClient;
use random::RandomRequest;

pub mod random {
    tonic::include_proto!("random");
}

#[derive(Serialize)]
struct ApiResponse<T> {
    code: u16,
    message: String,
    payload: T,
}

#[derive(Deserialize)]
struct TokenRequest {
    token: String,
}

async fn hello_handler() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse {
        code: 200,
        message: "Success".to_string(),
        payload: "Hello, Rust!",
    })
}

async fn token_handler(Json(payload): Json<TokenRequest>) -> Json<ApiResponse<String>> {
    let mut client = RandomServiceClient::connect("http://[::1]:50051")
        .await
        .expect("Failed to connect to worker");

    let response = client.get_random(RandomRequest {})
        .await
        .expect("Failed to get random value");

    let random_value = response.into_inner().value;

    Json(ApiResponse {
        code: 200,
        message: "Token received".to_string(),
        payload: format!("{}-{}", payload.token, random_value),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/token", post(token_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Master HTTP server running at http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
