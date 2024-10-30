use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestEnum {
    CreateGameRequest(CreateGameRequest),
    GetGamesRequest (GetGamesRequest),
    JoinGameRequest (JoinGameRequest),
    AuthorizeWebsocketConnectionRequest (AuthorizeWebsocketConnectionRequest),
    MakeMoveRequest (MakeMoveRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameRequest {
    pub user_id: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGamesRequest { }

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinGameRequest {
    pub game_id: Uuid,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeWebsocketConnectionRequest {
    pub game_id: Uuid,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MakeMoveRequest {
    pub game_id: Uuid,
    pub user_id: String,
    pub from: String,
    pub to: String,
}

//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreateGameRequest {
//     pub user_id: String,
//     pub color: String,
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct GetGamesRequest {}
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct JoinGameRequest {
//     pub game_id: Uuid,
//     pub user_id: String,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct AuthorizeWebsocketConnectionRequest {
//     pub game_id: Uuid,
//     pub user_id: String,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct MakeMoveRequest {
//     pub game_id: Uuid,
//     pub user_id: String,
//     pub from: String,
//     pub to: String,
// }


    // Add other requests here
    // LeaveGame { game_id: Uuid, player_name: String },
    // ChatMessage { game_id: Uuid, player_name: String, message: String },
    // StartGame { game_id: Uuid },
    // EndGame { game_id: Uuid },
    // GetGameState { game_id: Uuid },
    // Ping,