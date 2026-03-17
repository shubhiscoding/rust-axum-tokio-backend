use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::get};
use serde::Serialize;
#[derive(Serialize)]
struct Token {
    name: String,
    supply: u64,
}

#[derive(Serialize)]
struct  ErrorResponse {
    error: String,
}

struct AppState {
    tokens: HashMap<String, u64>,
}

#[tokio::main]
async fn main() {

    let mut tokens = HashMap::new();
    tokens.insert("MorphToken".to_string(), 1_000_000);
    tokens.insert("RustToken".to_string(), 500_000);

    let state = Arc::new(AppState {tokens});
    
    let app = Router::new()
    .route("/", get(root))
    .route("/token/{name}", get(get_token))
    .with_state(state);

    let listeners = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    println!("Hello, world!");
    axum::serve(listeners, app)
    .await
    .unwrap();
}

async fn root() -> &'static str {
    "Hello from Rust server!!"
}

async fn get_token( State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Result<Json<Token>, (StatusCode, Json<ErrorResponse>)> {
    match state.tokens.get(&name) {
        Some(supply) => {
            let token = Token {
                name,
                supply: *supply,
            };
            Ok(Json(token))
        }
        None => {
            let err_msg = ErrorResponse{error: String::from("Token Not Found")};
            return Err((StatusCode::NOT_FOUND, Json(err_msg)));
        }
    }
}