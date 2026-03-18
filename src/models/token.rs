use serde::Serialize;

#[derive(Serialize)]
pub struct Token {
    pub name: String,
    pub supply: u64,
}