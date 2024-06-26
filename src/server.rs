use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use futures_util::stream::SplitSink;
use chesslib::board::Board;
use futures_util::future::err;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::sync::Arc;
// use serde::de::Unexpected::Option;

struct Request_ {
    action: String,
    from: Option<String>,
    to: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Request {
    action: String,
    from: Option<String>,
    to: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    message: String,
}

struct Session {
    board: Option<Board>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            board: None,
        }
    }
    async fn create_game(&mut self, mut write: &SplitSink<WebSocketStream<TcpStream>, Message>) {
        let columns = "abcdefgh".to_string();
        let rows = "12345678".to_string();
        let board_size = 8;
        let board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

        let mut board = Board::new(columns, board_size, rows, board_size, board_fen);
        self.board = Some(board);

        let response = Response {
            status: "success".to_string(),
            message: "Game created successfully".to_string(),
        };
    }

    async fn make_move(&mut self, from: String, to: String) -> Result<(), String> {
        match self.board {
            None => Err("fail".to_string()),
            Some(ref mut board) => {
                board.make_move_str(from, to);
                Ok(())
            }
        }
    }

    async fn handle_connection(&mut self, stream: TcpStream) {
        let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");

        let (mut write, mut read) = ws_stream.split();

        while let Some(message) = read.next().await {
            let message = message.expect("Failed to read message");
            if message.is_text() {
                let request: Request = serde_json::from_str(message.to_text().unwrap()).expect("Invalid request format");

                let response = match request.action.as_str() {
                    "create_game" => {
                        self.create_game(&write).await;
                        Response {
                            status: "success".to_string(),
                            message: "Game created successfully".to_string(),
                        }
                    }
                    "make_move" => {
                        if let (Some(from), Some(to)) = (request.from, request.to) {
                            match self.make_move(from, to).await {
                                Ok(_) => Response {
                                    status: "success".to_string(),
                                    message: "Move made".to_string(),
                                },
                                Err(err) => Response {
                                    status: "error".to_string(),
                                    message: err,
                                }
                            }
                        } else {
                            Response {
                                status: "error".to_string(),
                                message: "Invalid move parameters".to_string(),
                            }
                        }
                    }
                    _ => Response {
                        status: "error".to_string(),
                        message: "Unknown action".to_string(),
                    }
                };

                let response_text = serde_json::to_string(&response).expect("Failed to serialize response");
                write.send(Message::Text(response_text)).await.expect("Failed to send response");
            }
        }
    }
}
pub async fn start_server() {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server is listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let session = Arc::new(Mutex::new(Session::new()));
        let session_clone = Arc::clone(&session);
        tokio::spawn(async move {
            session_clone.lock().await.handle_connection(stream).await;
        });
    }
}
