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

use crate::game::Game;
use crate::command::{parse_command, parse_command_option};


struct Session {
    game: Option<Game>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            game: None,
        }
    }
    fn create_game(&mut self) -> String {
        let columns = "abcdefgh".to_string();
        let rows = "12345678".to_string();
        let board_size = 8;
        let board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

        let mut board = Board::new(columns, board_size, rows, board_size, board_fen);
        let game = Game::create_game_from_board(board.clone());
        self.game = Some(game);
        println!("{}", board.board_to_string());
        "Game created successfully".to_string()
    }

    fn make_move(&mut self, params: HashMap<String, String>) -> String {
        match self.game.as_mut() {
            None => "Game does not exist".to_string(),
            Some(game) => {
                match game.get_board() {
                    None => "Game does not exist".to_string(),
                    Some(board) => {
                        // self.game.as_mut().unwrap().get_board().unwrap().make_move_str("e2".to_string(), "e4".to_string());
                        if let (Some(value_from), Some(value_to)) = (params.get("from"), params.get("to")) {
                            board.make_move_str(value_from.clone(), value_to.clone());
                            let result = "Made move: ".to_string()
                                + value_from.clone().as_str()
                                + " "
                                + value_to.clone().as_str()
                                + "\n"
                                + board.board_to_string().as_str();
                            println!("{}", result);
                            result
                        } else {
                            let result = "Wrong move: ".to_string();
                            println!("{}", result);
                            result
                        }
                    }
                }
            }
        }
        // let mut g = self.game.as_mut();
        // let mut s = g.as_mut().unwrap()
        //     .get_board()
        //     .unwrap()
        //     .get_position()
        //     .read()
        //     .unwrap()
        //     .get_square_by_coordinates(('e', '2'))
        //     .read()
        //     .unwrap()
        //     .square_to_str();
        // println!("{}", g.as_mut().unwrap().get_board().unwrap().board_to_string());
        // // println!("{}", s);
        // // let a = &self.game.unwrap()._board.unwrap().get_position().get_mut().unwrap().get_square_by_coordinates('e','2')
        // "".to_string()
    }
}

#[tokio::main]
pub async fn start_server() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (conn, _) = listener.accept().await?;
        let mut server = ServerBuilder::new().accept(conn).await?;
        let mut session = Arc::new(Mutex::new(Session::new()));

        while let Some(Ok(item)) = server.next().await {
            println!("Received: {item:?}");
            let response = match parse_command_option(item.as_text()) {
                Err(e) => e,
                Ok(command) => {
                    match command.command.as_str() {
                        "create_game" => session.lock().await.create_game(),
                        "make_move" => session.lock().await.make_move(command.parameters),
                        _ => "Unknown command".to_string(),
                    }
                }
            };
            let m = Message::text(response);
            server.send(m).await?;
        }
    }
}
