use axum::{
    response::IntoResponse,
    response::Response as AxumResponse,
};
use axum::extract::{Json, State};
use std::sync::{Arc, Mutex};


use crate::request::{
    CreateGameRequest,
    JoinGameRequest,
};
use crate::response::{
    CreateGameResponse,
    GetGamesResponse,
    JoinGameResponse,
    RequestFailedResponse,
};
use crate::game_repository::GameRepository;
use crate::game::Game;

pub async fn get_games_from_dict(State(game_repository): State<Arc<Mutex<GameRepository>>>) -> AxumResponse {
    println!("Get games request");
    let ids = game_repository.lock().unwrap().get_awaiting_games_from_dict();

    GetGamesResponse {game_ids: ids}.into_response()
}

pub async fn create_game(
    State(game_repository): State<Arc<Mutex<GameRepository>>>,
    Json(CreateGameRequest{user_id, color}): Json<CreateGameRequest>
) -> AxumResponse {
    println!("Create game request");
    let mut game = Game::new(user_id, color);
    match game_repository.lock().unwrap().add_game(game.clone()) {
        Ok(message) => CreateGameResponse {
            game_id: game.get_game_id(),
            message
        }.into_response(),
        Err(error) => RequestFailedResponse {
            message: error,
        }.into_response()
    }
}

pub async fn join_game(State(game_repository): State<Arc<Mutex<GameRepository>>>, Json(request): Json<JoinGameRequest>) -> AxumResponse {
    println!("Join game request");
    let JoinGameRequest{game_id, user_id} = request;
    match game_repository.lock().unwrap().get_game_by_id(game_id.clone()) {
        Some(game) => {
            match (game.get_users().0.unwrap() != user_id, game.get_users().1) {
                (true, None) => game.set_user(None, Some(user_id)),
                _ => (),
            }
            match game.get_users() {
                (Some(_), _) => JoinGameResponse {
                    game_id,
                    message: "Joined game".to_string()
                }.into_response(),
                (_, Some(_)) => JoinGameResponse {
                    game_id,
                    message: "Joined game".to_string()
                }.into_response(),
                (_, None) => RequestFailedResponse {
                    message: "Could not join game".to_string(),
                }.into_response(),
            }
        },
        _ => RequestFailedResponse { message: "Game not found".to_string() }.into_response()
    }
}

