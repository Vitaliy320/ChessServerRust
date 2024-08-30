use std::collections::HashMap;
use std::ops::DerefMut;
use uuid::Uuid;
use postgres::{Client, NoTls, Error as PostgresError};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;
// use tokio_tungstenite::tungstenite::protocol::Role::Client;
use crate::game::Game;
use crate::game::GameStatus;

// db url
pub fn set_db() {
    let db_url = std::env::var("DATABASE_URL");
    match db_url {
        Ok(url) => println!("DB url: {}", url),
        _ => println!("Fail"),
    }

    const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
    const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

}














pub struct GameRepository {
    games_dict: HashMap<Uuid, Game>,
}

impl GameRepository {
    pub fn new() -> Self {
        GameRepository {
            games_dict: HashMap::new(),
        }
    }
    pub fn add_game(
        &mut self,
        game: Game,
        ) -> Result<String, String> {

        self.games_dict.insert(game.get_game_id(), game);
        Ok("Game added successfully".to_string())
    }

    pub fn get_game_by_id(&mut self, game_id: Uuid) -> Option<&mut Game> {
        self.games_dict.get_mut(&game_id)
    }

    pub fn get_awaiting_games_from_dict(&self) -> Vec<Uuid> {
        let ids = self.games_dict.clone().iter()
            .filter_map(|(uuid, game)| {
                if matches!(game.get_game_status(), GameStatus::AwaitingOpponent) {
                    Some(*uuid)
                } else {
                    None
                }
            })
            .collect();
        ids
    }
}