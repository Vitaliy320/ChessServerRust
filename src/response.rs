use std::collections::HashMap;
use std::net::SocketAddr;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Response as AxumResponse, IntoResponse};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::game::Game;
use crate::game_status::GameStatus;
use crate::game_end_condition::GameEndCondition;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    CreateGameResponse { game_id: Uuid, message: String, },
    GetGamesResponse { game_ids: Vec<Uuid>, },
    JoinGameResponse { game_id: Uuid, message: String, },
    AuthorizeWebsocketConnectionResponse {
        game_id: Uuid,
        user_id: String,
        connection_id: SocketAddr,
        board: HashMap<String, (String, Vec<String>)>,
        message: String,
    },
    MakeMoveResponse {
        game_id: Uuid,
        message: String,
        columns: String,
        rows: String,
        board: HashMap<String, (String, Vec<String>)>,
        game_status: GameStatus,
        game_end_condition: GameEndCondition,
    },
    RequestFailedResponse { message: String, }
}

impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        match self {
            Response::CreateGameResponse { game_id, message } => {
                let body = Json(serde_json::json!({
                    "game_id": game_id,
                    "message": message,
                }));
                (StatusCode::OK, body).into_response()
            },
            Response::GetGamesResponse { game_ids } => {
                let body = Json(serde_json::json!({
                    "game_ids": game_ids,
                }));
                (StatusCode::OK, body).into_response()
            },
            Response::JoinGameResponse { game_id, message } => {
                let body = Json(serde_json::json!({
                    "game_id": game_id,
                    "message": message,
                }));
                (StatusCode::OK, body).into_response()
            },
            Response::AuthorizeWebsocketConnectionResponse { game_id, user_id, connection_id, board, message } => {
                let body = Json(serde_json::json!({
                    "game_id": game_id,
                    "user_id": user_id,
                    "connection_id": connection_id,
                    "board": board,
                    "message": message,
                }));
                (StatusCode::OK, body).into_response()
            },
            Response::MakeMoveResponse {
                game_id,
                message,
                columns,
                rows,
                board,
                game_status,
                game_end_condition,
            } => {
                let body = Json(serde_json::json!({
                    "game_id": game_id,
                    "message": message,
                    "columns": columns,
                    "rows": rows,
                    "board": board,
                    "game_status": game_status.to_string(),
                    "game_end_condition": game_end_condition.to_string(),
                }));
                (StatusCode::OK, body).into_response()
            },
            Response::RequestFailedResponse { message } => {
                let body = Json(serde_json::json!({
                    "message": message,
                }));
                (StatusCode::OK, body).into_response()
            },
        }
    }
}
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreateGameResponse {
//     pub game_id: Uuid,
//     pub message: String,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct GetGamesResponse {
//     pub game_ids: Vec<Uuid>,
// }
//
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct JoinGameResponse {
//     pub game_id: Uuid,
//     pub message: String,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct MakeMoveResponse {
//     pub game_id: Uuid,
//     pub message: String,
//     pub columns: String,
//     pub rows: String,
//     pub board: HashMap<String, (String, Vec<String>)>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct RequestFailedResponse {
//     pub message: String,
// }
//
// macro_rules! impl_into_response {
//     ($($struct_name:ident),*) => {
//         $(
//             impl IntoResponse for $struct_name {
//                 fn into_response(self) -> AxumResponse {
//                     let body = Json(self);
//                     (StatusCode::OK, body).into_response()
//                 }
//             }
//         )*
//     };
// }
//
// impl_into_response!(
//     CreateGameResponse,
//     MakeMoveResponse,
//     GetGamesResponse,
//     JoinGameResponse,
//     RequestFailedResponse
// );
//

    // // Add other responses here
    // GameLeft { game_id: Uuid, message: String },
    // ChatReceived { game_id: Uuid, player_name: String, message: String },
    // GameStarted { game_id: Uuid, message: String },
    // GameEnded { game_id: Uuid, message: String },
    // GameState { game_id: Uuid, state: String },
    // Pong,
    // Error { message: String },
