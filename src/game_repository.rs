use std::collections::HashMap;
use std::ops::DerefMut;
use uuid::Uuid;
use tokio_postgres::{Client, NoTls, Error as PostgresError};
use tokio::sync::Mutex as tokioMutex;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
// use std::env;
use std::fmt::format;
use std::sync::{Arc, Mutex};
use diesel::RunQueryDsl;
// use tokio_tungstenite::tungstenite::protocol::Role::Client;
use crate::game::Game;
use crate::game::GameStatus;
use dotenv::dotenv;
use config::Config;
use postgres::types::ToSql;
use futures_util::future::join_all;
use futures_util::TryFutureExt;
use crate::user::User;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

pub struct GameRepository {
    db_client: Option<Client>,
    games_dict: HashMap<Uuid, Game>,
}

impl GameRepository {
    pub fn new() -> Self {
        dotenv().ok();
        GameRepository {
            db_client: None,
            games_dict: HashMap::new(),
        }
    }

    pub async fn connect_to_db(&mut self) {
        let db_url = std::env::var("DATABASE_URL");
        let db_url: String = match db_url {
            Ok(url) => {
                println!("DB url: {}", &url);
                url
            }
            _ => {
                println!("Fail");
                "".to_string()
            },
        };

        let client = tokio_postgres::connect(db_url.as_str(), NoTls).await;
        match client {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("db connection error: {e}");
                    }
                });

                self.db_client = Some(client);
                println!("Connected to db");
            },
            _ => println!("Could not connect to db"),
        }

        let names = ["Steve", "John", "Paul", "Eric", "Glenn"];

        let users: Vec<User> = names
            .iter()
            .map(|name| {
                User {
                    user_id: "".to_string(),
                    name: name.to_string(),
                    email: format!("{}@gmail.com", name),
                }
            }).collect();

        let _ = self.add_users_batch_to_users(users).await;

        let users = self.get_users().await;
        match users {
            Ok(users) => {
                users.iter().for_each(|user| {
                    println!("User id: {}, name: {}, email: {}", user.user_id, user.name, user.email);
                });
            },
            _ => (),
        }
    }

    pub async fn add_user_to_users(&self, user: User) -> Result<(), String> {
        match self.db_client {
            Some(ref db_client) => {
                let result = db_client.execute(
                    "INSERT INTO users (name, email) VALUES ($1, $2)",
                &[&user.name, &user.email]).await;
                match result {
                    Ok(_) => Ok(()),
                    _ => Err("Could not add user".to_string()),
                }
            },
            _ => Err("Could not connect to the database".to_string()),
        }
    }

    pub async fn add_users_batch_to_users(&self, users: Vec<User>) -> Result<(), String> {
        match self.db_client {
            Some(ref db_client) => {
                let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut query = "INSERT INTO users (name, email) VALUES ".to_string();
                let users_len = users.len();
                for i in 0..users_len {
                    params.push(&users.get(i).unwrap().name);
                    params.push(&users.get(i).unwrap().email);

                    query.push_str(format!("(${}, ${})", i * 2 + 1, i * 2 + 2).as_str());
                    if i < users_len - 1 {
                        query.push_str(", ");
                    }
                }
                let result = db_client.execute(&query,
                                               &params).await;
                match result {
                    Ok(_) => Ok(()),
                    _ => Err("Could not add users".to_string()),
                }
            },
            _ => Err("Could not connect to the database".to_string()),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, String> {
        match self.db_client {
            Some(ref db_client) => {
                match db_client.query("SELECT * FROM users", &[]).await {
                    Ok(rows) => {
                        let users: Vec<User> = rows.iter().map(|row| {
                            let id: i32 = row.get(0);
                            User {
                                user_id: id.to_string(),
                                name: row.get(1),
                                email: row.get(2),
                            }
                        }).collect();
                        Ok(users)
                    },
                    _ => Err("Could not connect to the database".to_string()),
                }
            },
            None => Err("Could not connect to db".to_string()),
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