use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
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

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/token/{name}", get(get_token));

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

async fn get_token(Path(name): Path<String>) -> Result<Json<Token>, (StatusCode, Json<ErrorResponse>)> {
    let supply = match name.as_str() {
        "MorphToken" => 1000000,
        "RustToken" => 500000,
        _ => {
            let err_msg = ErrorResponse{error: String::from("Token Not Found")};
            return Err((StatusCode::NOT_FOUND, Json(err_msg)));
        }
    };
    let token = Token {
        name,
        supply
    };
    Ok(Json(token))
}