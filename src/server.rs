use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use chesslib::board::Board;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitSink;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tokio_websockets::{Error, Message, ServerBuilder};
use tokio::sync::Mutex;
// use tokio_tungstenite::tungstenite::http::{request, response};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::command::{parse_command, parse_command_option};
use crate::game_repository::GameRepository;
use crate::request::Request;
use crate::response::Response;

pub struct Server {
    game: Option<Game>,
    game_repository: GameRepository,
}

impl Server {
    pub fn new() -> Server {
        Server {
            game: None,
            game_repository: GameRepository::new(),
        }
    }
    fn create_game(&mut self, user_id: String, color: String) -> Response {
        self.game = Some(Game::new(
            Uuid::new_v4(),
            user_id.clone(),
            color.clone(),
        ));

        match self.game_repository.add_game(
            self.game.clone().unwrap().get_game_id(),
            self.game.clone().unwrap(),
            ) {

            Ok(_) => {
                println!("Game created successfully");
                Response::GameCreated {
                    game_id: Some(self.game.clone().unwrap().get_game_id()),
                    message: "Game created successfully".to_string(),
                }
            },
            Err(_) => {
                println!("Could not create a game");
                Response::GameCreated {
                    game_id: None,
                    message: "Could not create a game".to_string(),
                }
            }
        }
    }

    fn make_move(&mut self, game_id: Uuid, user_id: String, from: String, to: String) -> Response {
        match self.game_repository.get_game_by_id(game_id) {
            None => Response::MoveMade {
                game_id,
                message: "Game does not exist".to_string(),
                board: HashMap::new(),
            },
            Some(game) => {
                match game.get_board() {
                    None => Response::MoveMade {
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
                        Response::MoveMade {
                            game_id,
                            message: format!("Made move from {} to {}", from, to),
                            board: HashMap::new(),
                        }
                    }
                }
            },
        }
    }


    pub async fn handle_request(&mut self, request: Request) -> Response {
        match request {
            Request::CreateGame { user_id, color } => {
                self.create_game(user_id, color)
            },
            Request::MakeMove { game_id, user_id, from, to} => {
                self.make_move(game_id, user_id, from, to)
            },
        }
    }
}


#[tokio::main]
pub async fn start_server() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (conn, _) = listener.accept().await?;
        let mut ws = ServerBuilder::new().accept(conn).await?;
        let mut server = Arc::new(Mutex::new(Server::new()));

        while let Some(Ok(item)) = ws.next().await {
            println!("Received: {item:?}");
            let response = match item.as_text() {
                None => "Could not handle request".to_string(),
                Some(text) => {
                    match serde_json::from_str::<Request>(text) {
                        Err(_) => "Could not handle request".to_string(),
                        Ok(request) => {
                            let response = server.clone().lock().await.handle_request(request).await;
                            serde_json::to_string(&response).unwrap()
                        },
                    }
                }
            };
            let m = Message::text(response);
            ws.send(m).await?;
        }
    }
}
