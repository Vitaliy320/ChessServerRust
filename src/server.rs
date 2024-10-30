use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use std::thread;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio::net::TcpListener;
use tokio_websockets::ServerBuilder;
use crate::game::Game;
use crate::game_repository::GameRepository;
use crate::http_server::{get_games_from_dict, create_game, join_game};
use futures_util::{SinkExt, StreamExt};
use crate::connection_manager::ConnectionManager;
// use crate::websocket_server::run_websocket_server;
use crate::websocket_server_new::run_websocket_server_new;
use crate::game_manager::GameManager;


pub struct SharedState {
    pub game_manager: Arc<RwLock<GameManager>>,
    pub connection_manager: Box<ConnectionManager>,
}

pub async fn run_server() {
    // Channel for broadcasting messages to WebSocket clients
    let (tx, _rx) = broadcast::channel::<String>(100);
    let tx_ws = tx.clone();

    // let game_repository = Arc::new(Mutex::new(GameRepository::new()));
    let mut game_repository = GameRepository::new();
    game_repository.connect_to_db().await;

    let game_manager = Arc::new(RwLock::new(GameManager::new(game_repository)));

    // let game_manager_clone = Arc::clone(&game_manager);
    let api_handle = tokio::spawn(run_http_server(Arc::clone(&game_manager)));
    let websocket_handle = tokio::spawn(run_websocket_server_new(Arc::clone(&game_manager)));
    let _ = tokio::join!(
        api_handle,
        websocket_handle);
}

async fn run_http_server(game_manager: Arc<RwLock<GameManager>>) {
    let api_router = Router::new()
        .route("/get_games", get(get_games_from_dict))
        .route("/create_game", post(create_game))
        .route("/join_game", put(join_game))
        .with_state(game_manager);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("HTTP server started");
    axum::serve(listener, api_router).await.unwrap();
}
