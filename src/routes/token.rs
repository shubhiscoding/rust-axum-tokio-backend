use std::sync::{Arc, Mutex};

use axum::{Json, extract::{Path, State}, http::StatusCode};

use crate::models::{AppState, ErrorResponse, MintRequest, Token, BurnRequest};

pub async fn mint(
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

pub async fn get_token( 
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

pub async fn burn(
    State(state): State<Arc<Mutex<AppState>>>, 
    Json(payload): Json<BurnRequest>
) -> Result<Json<Token>, (StatusCode, Json<ErrorResponse>)> {  
    let token = {
        let mut state = state.lock().unwrap();
        match state.tokens.get_mut(&payload.name) {
            Some(curr) => {
                if *curr < payload.amount {
                    let err_msg = ErrorResponse{error: String::from("Insufficient Supply")};
                    return Err((StatusCode::BAD_REQUEST, Json(err_msg)));
                }
                *curr -= payload.amount;
                Token {
                    name: payload.name,
                    supply: *curr
                }
            },
            None => {
                let err_msg = ErrorResponse{error: String::from("Token not found")};
                return Err((StatusCode::NOT_FOUND, Json(err_msg)));
            }
        }
    };
    Ok(Json(token))
}