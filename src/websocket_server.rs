use std::{sync::Arc, sync::Mutex};
use std::collections::HashMap;
use std::ops::Deref;
use crate::chess_engine::board::Board;
use diesel::serialize::ToSql;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc::unbounded_channel;
use tokio_websockets::ServerBuilder;
use uuid::Uuid;
use crate::game_repository::GameRepository;
use crate::request::MakeMoveRequest;
use crate::response::MakeMoveResponse;

pub async fn run_websocket_server(game_repository: Arc<Mutex<GameRepository>>) {
    let ws_listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("Websocket server started");
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
                        match serde_json::from_str::<MakeMoveRequest>(text) {
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


pub async fn handle_request(request: MakeMoveRequest, game_repository: Arc<Mutex<GameRepository>>) -> MakeMoveResponse {
    let MakeMoveRequest{ game_id, user_id, from, to} = request;
    make_move(game_repository, game_id, user_id, from, to).await
}

async fn make_move(game_repository: Arc<Mutex<GameRepository>>, game_id: Uuid, user_id: String, from: String, to: String) -> MakeMoveResponse {
    let mut game_repository_guard = game_repository.lock().unwrap();

    match game_repository_guard.get_game_by_id(game_id) {
        None => MakeMoveResponse {
            game_id,
            message: "Game does not exist".to_string(),
            columns: "".to_string(),
            rows: "".to_string(),
            board: HashMap::new(),
        },
        Some(game) => {
            match game.get_board() {
                None => MakeMoveResponse {
                    game_id,
                    message: "Game does not exist".to_string(),
                    columns: "".to_string(),
                    rows: "".to_string(),
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
                    MakeMoveResponse {
                        game_id,
                        message: format!("Made move from {} to {}", from, to),
                        columns: board.get_columns(),
                        rows: board.get_rows(),
                        board: board_to_dict(&board),
                    }
                }
            }
        },
    }
}
fn board_to_dict(board: &Board) -> HashMap<String, (String, Vec<String>)> {
    let mut dict: HashMap<String, (String, Vec<String>)> = HashMap::new();
    // todo: add calculation of possible moves for each piece in given position

    let position = board.get_position();
    let position = &position.lock().unwrap();

    let possible_coordinates: Vec<String> = position.get_squares().keys().cloned().collect();

    for (coordinates, square) in position.get_squares() {
        let square = square.lock().unwrap();
        let piece = square.get_piece();
        match piece {
            Some(p) => {
                let piece_possible_coordinates = p.lock().unwrap().get_possible_moves();
                dict.insert(coordinates.deref().to_string(), (p.lock().unwrap().get_symbol(), possible_coordinates.clone()));
            },
            _ => (),
        }
    }
    dict
}
