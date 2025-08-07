use axum::{
    routing::{get, post},
    Json, Router
};
use axum::{http::StatusCode};
use serde::{Deserialize, Serialize};

use hmac::{Hmac, Mac};
use sha2::{Sha256};
type HmacSha256 = Hmac<Sha256>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
    .route("/signature", post(generate_signature))
    .route("/", get(|| async {"Hello World"}));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_signature(Json(req): Json<SignatureRequest>) -> (StatusCode, Json<SignatureResponse>){
    let private_key = std::env::var("PRIVATE_KEY").unwrap();
    let merchant_code = std::env::var("MERCHANT_CODE").unwrap();
    

    let mut mac = HmacSha256::new_from_slice(private_key.as_bytes()).unwrap();
    mac.update(format!("{}{}{}", merchant_code, req.merchant_ref, req.amount).as_bytes());
    let result = mac.finalize().into_bytes();

    (StatusCode::CREATED, Json(SignatureResponse {
        signature: hex::encode(result)
    }))
}

#[derive(Deserialize)]
struct SignatureRequest {
    merchant_ref: String,
    amount: u64
}

#[derive(Serialize)]
struct SignatureResponse {
    signature: String,
}