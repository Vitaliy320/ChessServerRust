use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tokio::sync::{broadcast};
use tokio::net::TcpListener;
use tokio_websockets::ServerBuilder;
use crate::game::Game;
use crate::game_repository::GameRepository;
use crate::http_server::{get_games_from_dict, create_game, join_game};
use futures_util::{SinkExt, StreamExt};
use crate::websocket_server::run_websocket_server;


pub async fn run_server() {
    // Channel for broadcasting messages to WebSocket clients
    let (tx, _rx) = broadcast::channel::<String>(100);
    let tx_ws = tx.clone();

    let game_repository = Arc::new(Mutex::new(GameRepository::new()));
    game_repository.lock().unwrap().connect_to_db().await;
    // set_db().await;
    // game_repository.lock().unwrap().connect_to_db().await;
    let game_repository_clone = Arc::clone(&game_repository);
    let api_handle = tokio::spawn(run_http_server(game_repository));
    let websocket_handle = tokio::spawn(run_websocket_server(game_repository_clone));
    let _ = tokio::join!(api_handle, websocket_handle);
}

async fn run_http_server(game_repository: Arc<Mutex<GameRepository>>) {
    let api_router = Router::new()
        .route("/get_games", get(get_games_from_dict))
        .route("/create_game", post(create_game))
        .route("/join_game", put(join_game))
        .with_state(Arc::clone(&game_repository));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("HTTP server started");
    axum::serve(listener, api_router).await.unwrap();
}
