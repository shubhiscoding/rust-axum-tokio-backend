use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MintRequest {
    pub name: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct BurnRequest {
    pub name: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct  ErrorResponse {
    pub error: String,
}