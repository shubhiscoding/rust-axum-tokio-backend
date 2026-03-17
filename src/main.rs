use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root));

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