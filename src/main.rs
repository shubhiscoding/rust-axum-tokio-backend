use std::{collections::HashMap, sync::{Arc, RwLock}};
use axum::{ Router, routing::{get, post}};

mod models;
use models::AppState;
mod routes;
use routes::mint;
use routes::get_token;

use crate::routes::burn;

#[tokio::main]
async fn main() {

    let mut tokens = HashMap::new();
    tokens.insert("MorphToken".to_string(), 1_000_000);
    tokens.insert("RustToken".to_string(), 500_000);

    let state = Arc::new(RwLock::new(AppState {tokens}));
    
    let app = Router::new()
    .route("/", get(root))
    .route("/token/{name}", get(get_token))
    .route("/mint", post(mint))
    .route("/burn", post(burn))
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
