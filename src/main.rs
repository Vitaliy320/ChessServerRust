mod server;
mod game;
mod game_repository;
mod response;
mod request;
mod http_server;
mod websocket_server;
mod chess_engine;
mod user;
mod game_status;
mod event_service;
mod game_manager;
mod websocket_server_new;
mod connection_manager;
mod game_end_condition;

use chess_engine::board::Board;
use std::io::{self, Write};
use tokio::net::TcpListener;
use crate::chess_engine::coordinates::Coordinates;
use dotenv::dotenv;

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
    dotenv().ok();
    // let mut foo = Foo{};
    // foo.foo().await;

    // let _ = websocket_server_new::run_server().await;

    let _ = server::run_server().await;
}
