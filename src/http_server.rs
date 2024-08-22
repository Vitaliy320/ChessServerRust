use axum::{
    response::IntoResponse,
    response::Response as AxumResponse,
};
use axum::extract::{Json, State};
use std::sync::{Arc, Mutex};


use crate::request::Request;
use crate::response::Response;
use crate::game_repository::GameRepository;
use crate::game::Game;

pub async fn get_games_from_dict(State(game_repository): State<Arc<Mutex<GameRepository>>>) -> AxumResponse {
    let ids = game_repository.lock().unwrap().get_awaiting_games_from_dict();

    Response::GetGamesResponse {game_ids: ids}.into_response()
}

pub async fn create_game(State(game_repository): State<Arc<Mutex<GameRepository>>>, Json(request): Json<Request>) -> AxumResponse {
    match request {
        Request::CreateGame { user_id, color} => {
            let mut game = Game::new(user_id, color);
            match game_repository.lock().unwrap().add_game(game.clone()) {
                Ok(message) => Response::CreateGameResponse {
                    game_id: game.get_game_id(),
                    message
                }.into_response(),
                Err(error) => Response::RequestFailedResponse {
                    message: error,
                }.into_response()
            }
        },
        _ => Response::RequestFailedResponse { message: "Could not create game".to_string() }.into_response(),
    }
}

pub async fn join_game(State(game_repository): State<Arc<Mutex<GameRepository>>>, Json(request): Json<Request>) -> AxumResponse {
    match request {
        Request::JoinGame { game_id, user_id } => {
            match game_repository.lock().unwrap().get_game_by_id(game_id.clone()) {
                Some(game) => {
                    match (game.get_users().0.unwrap() != user_id, game.get_users().1) {
                        (true, None) => game.set_user(None, Some(user_id)),
                        _ => (),
                    }
                    match game.get_users() {
                        (Some(_), _) => Response::JoinGameResponse {
                            game_id,
                            message: "Joined game".to_string()
                        }.into_response(),
                        (_, Some(_)) => Response::JoinGameResponse {
                            game_id,
                            message: "Joined game".to_string()
                        }.into_response(),
                        (_, None) => Response::RequestFailedResponse {
                            message: "Could not join game".to_string(),
                        }.into_response(),
                    }
                },
                _ => Response::RequestFailedResponse { message: "Game not found".to_string() }.into_response()
            }
        },
        _ => Response::RequestFailedResponse { message: "".to_string() }.into_response(),
    }
}

