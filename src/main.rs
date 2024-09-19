mod server;
mod game;
mod game_repository;
mod response;
mod request;
mod http_server;
mod websocket_server;
mod chess_engine;
mod user;

use chess_engine::board::Board;
use std::io::{self, Write};
use tokio::net::TcpListener;

struct Foo;
impl Foo {
    pub async fn foo(&mut self) {
        let users = ["Steve", "Joe", "Paul"];

        // let mut results: Vec<Result<(), &str>> = Vec::new();
        let mut results: Vec<Result<(), String>> = Vec::new();

        for user in users {
            let res = self.bar(user.to_string()).await;
            results.push(res);
        }
    }

    // pub async fn bar(&mut self, _: String) -> Result<(), &str> {
    pub async fn bar(&mut self, _: String) -> Result<(), String> {
        Ok(())
    }
}
#[tokio::main]
async fn main() {
    // let mut foo = Foo{};
    // foo.foo().await;
    let a = server::run_server().await;
}
// fn main() {
    // let columns = "abcdefgh";
    // let rows = "12345678";
    // let board_size = 8;
    // let board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    //
    // let mut board = Board::new(
    //     String::from(columns),
    //     board_size,
    //     String::from(rows),
    //     board_size,
    //     board_fen.to_string(),
    // );
    //
    // println!("{}", board.board_to_string());
    // board.make_move_str("e2".to_string(), "e4".to_string());
    // println!("{}", board.board_to_string());
    // board.make_move_str("d7".to_string(), "d5".to_string());
    // println!("{}", board.board_to_string());
    // board.make_move_str("e4".to_string(), "d5".to_string());
    // println!("{}", board.board_to_string());
    // board.make_move_str("d8".to_string(), "d5".to_string());
    // println!("{}", board.board_to_string());
    //
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
// }