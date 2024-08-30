use std::collections::HashMap;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Response as AxumResponse, IntoResponse};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameResponse {
    pub game_id: Uuid,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MakeMoveResponse {
    pub game_id: Uuid,
    pub message: String,
    pub columns: String,
    pub rows: String,
    pub board: HashMap<String, (String, Vec<String>)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGamesResponse {
    pub game_ids: Vec<Uuid>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct JoinGameResponse {
    pub game_id: Uuid,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestFailedResponse {
    pub message: String,
}

macro_rules! impl_into_response {
    ($($struct_name:ident),*) => {
        $(
            impl IntoResponse for $struct_name {
                fn into_response(self) -> AxumResponse {
                    let body = Json(self);
                    (StatusCode::OK, body).into_response()
                }
            }
        )*
    };
}

impl_into_response!(
    CreateGameResponse,
    MakeMoveResponse,
    GetGamesResponse,
    JoinGameResponse,
    RequestFailedResponse
);


    // // Add other responses here
    // GameLeft { game_id: Uuid, message: String },
    // ChatReceived { game_id: Uuid, player_name: String, message: String },
    // GameStarted { game_id: Uuid, message: String },
    // GameEnded { game_id: Uuid, message: String },
    // GameState { game_id: Uuid, state: String },
    // Pong,
    // Error { message: String },
