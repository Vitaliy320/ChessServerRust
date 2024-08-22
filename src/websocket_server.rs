use std::{sync::Arc, sync::Mutex};
use std::collections::HashMap;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_websockets::ServerBuilder;
use uuid::Uuid;
use crate::game_repository::GameRepository;
use crate::request::Request;
use crate::response::Response;

pub async fn run_websocket_server(game_repository: Arc<Mutex<GameRepository>>) {
    let ws_listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    loop {
        let (conn, _) = ws_listener.accept().await.unwrap();
        println!("Client connected");

        let game_repository_clone = Arc::clone(&game_repository);

        let _ = tokio::spawn(async move {
            let mut ws = ServerBuilder::new().accept(conn).await.unwrap();
            while let Some(Ok(item)) = ws.next().await {
                println!("Received: {item:?}");
                let response = match item.as_text() {
                    None => "Could not handle request".to_string(),
                    Some(text) => {
                        match serde_json::from_str::<Request>(text) {
                            Err(_) => "Could not handle request".to_string(),
                            Ok(request) => {
                                let response = handle_request(request, Arc::clone(&game_repository_clone)).await;
                                serde_json::to_string(&response).unwrap()
                            },
                        }
                    }
                };
                let m = tokio_websockets::Message::text(response);
                ws.send(m).await.unwrap();
            }
        });
    }
}


pub async fn handle_request(request: Request, game_repository: Arc<Mutex<GameRepository>>) -> Response {
    match request {
        Request::MakeMove { game_id, user_id, from, to} => {
            make_move(game_repository, game_id, user_id, from, to).await
        },
        _ => Response::RequestFailedResponse { message: "Request not implemented yet".to_string() },
    }
}

async fn make_move(game_repository: Arc<Mutex<GameRepository>>, game_id: Uuid, user_id: String, from: String, to: String) -> Response {
    let mut mutex_guard = game_repository.lock().unwrap();

    match mutex_guard.get_game_by_id(game_id) {
        None => Response::MakeMoveResponse {
            game_id,
            message: "Game does not exist".to_string(),
            board: HashMap::new(),
        },
        Some(game) => {
            match game.get_board() {
                None => Response::MakeMoveResponse {
                    game_id,
                    message: "Game does not exist".to_string(),
                    board: HashMap::new(),
                },
                Some(board) => {
                    // self.game.as_mut().unwrap().get_board().unwrap().make_move_str("e2".to_string(), "e4".to_string());
                    board.make_move_str(from.clone(), to.clone());
                    let result = "Made move: ".to_string()
                        + from.clone().as_str()
                        + " "
                        + to.clone().as_str()
                        + "\n"
                        + board.board_to_string().as_str();
                    println!("{}", result);
                    Response::MakeMoveResponse {
                        game_id,
                        message: format!("Made move from {} to {}", from, to),
                        board: HashMap::new(),
                    }
                }
            }
        },
    }
}
