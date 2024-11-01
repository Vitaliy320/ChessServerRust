use axum::{
    response::IntoResponse,
    response::Response as AxumResponse,
    debug_handler,
};
use axum::extract::{Json, State};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio_postgres::types::ToSql;
use crate::connection_manager::ConnectionManager;
use crate::request::{
    RequestEnum,
    GetGamesRequest,
    CreateGameRequest,
    JoinGameRequest,
    AuthorizeWebsocketConnectionRequest,
    MakeMoveRequest,
};
use crate::response::Response;

use crate::game_repository::GameRepository;
use crate::game_manager::GameManager;
use crate::game::Game;
use crate::event_service::EventService;
use crate::server::SharedState;


#[debug_handler]
pub async fn get_games_from_dict(
    State(game_manager): State<Arc<RwLock<GameManager>>>,
) -> AxumResponse {
    println!("Get games request");
    let ids = game_manager.read().await.get_awaiting_games();

    Response::GetGamesResponse {game_ids: ids}.into_response()
}


#[debug_handler]
pub async fn create_game(
    State(mut game_manager): State<Arc<RwLock<GameManager>>>,
    Json(request): Json<CreateGameRequest>,
) -> AxumResponse {
    let CreateGameRequest { user_id, color } = request;
    println!("Create game request");
    let mut game_manager_lock = game_manager.write().await;
    let game = Game::new(user_id.clone(), color);
    let response = game_manager_lock.add_game_to_games(game.clone()).await;
    match response {
        Ok(game_id) => {
            let _ = game_manager_lock.connection_manager.add_connection(
                &game.get_game_id(),
                &user_id,
                None,
                None,
            );
            Response::CreateGameResponse {
                game_id,
                message: "Game created successfully".to_string(),
            }.into_response()
        },
        Err(message) => Response::RequestFailedResponse {
            message,
        }.into_response()
    }
}


#[debug_handler]
pub async fn join_game(
    State(mut game_manager): State<Arc<RwLock<GameManager>>>,
    Json(request): Json<JoinGameRequest>
) -> AxumResponse {
    println!("Join game request");
    let JoinGameRequest { game_id, user_id } = request;
    {
        let _ = game_manager.write().await.connection_manager.add_connection(
            &game_id,
            &user_id,
            None,
            None,
        );
    }

    let mut game_manager_lock = game_manager.write().await;

    let game_result = game_manager_lock.get_game_by_id(&game_id).await;
    let mut game = match game_result {
        Ok(game) => game,
        Err(_) => {
            return Response::RequestFailedResponse {
                message: "Could not join game".to_string(),
            }.into_response();
        },
    };

    if game.get_users().0.unwrap() != user_id && game.get_users().1.is_none() {
        game.set_user(None, Some(user_id.clone()));
    }

    match game.get_users() {
        (Some(_), _) | (_, Some(_)) => Response::JoinGameResponse {
            game_id,
            message: "Joined game".to_string()
        }.into_response(),
        (_, None) => Response::RequestFailedResponse {
            message: "Could not join game".to_string(),
        }.into_response(),
    }
}

