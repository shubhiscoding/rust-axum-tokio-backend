use std::{collections::HashMap, sync::{Arc, Mutex}};
use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, post}};
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize)]
struct Token {
    name: String,
    supply: u64,
}

#[derive(Deserialize)]
struct MintRequest {
    name: String,
    amount: u64,
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

    let state = Arc::new(Mutex::new(AppState {tokens}));
    
    let app = Router::new()
    .route("/", get(root))
    .route("/token/{name}", get(get_token))
    .route("/mint", post(mint))
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

async fn get_token( 
    State(state): State<Arc<Mutex<AppState>>>, 
    Path(name): Path<String>
) -> Result<Json<Token>, (StatusCode, Json<ErrorResponse>)> {
    let state = state.lock().unwrap();

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

async fn mint(
    State(state): State<Arc<Mutex<AppState>>>, 
    Json(payload): Json<MintRequest>
) -> Result<Json<Token>, (StatusCode, Json<ErrorResponse>)> {    
    let token = {
        let mut state = state.lock().unwrap();

        match state.tokens.get_mut(&payload.name) {
            Some(supply) => {
                *supply += payload.amount;

                Token {
                    name: payload.name,
                    supply: *supply,
                }
            }
            None => {
                return Err((StatusCode::NOT_FOUND, Json(ErrorResponse {
                    error: "Token Not Found".to_string(),
                })));
            }
        }
    };

    Ok(Json(token))
}