use std::collections::HashMap;
use std::ops::DerefMut;
use uuid::Uuid;
use tokio_postgres::{Client, NoTls};

use diesel::RunQueryDsl;
use crate::game::Game;
use crate::game_status::GameStatus;

use postgres::types::ToSql;
use futures_util::future::join_all;
use futures_util::TryFutureExt;
use serde_json::to_string;
use tokio_tungstenite::tungstenite::client;
use crate::chess_engine::board::Board;

use crate::chess_engine::piece::PieceEnum;
use crate::user::User;
use crate::chess_engine::coordinates::Coordinates;


pub struct GameRepository {
    db_client: Option<Client>,
    games_dict: HashMap<Uuid, Game>,
}

impl GameRepository {
    pub fn new() -> Self {
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
            Err(e) => {
                println!("{}", e.to_string());
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
            Err(e) => {
                println!("{}", e.to_string());
                println!("Could not connect to db")
            },
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

        // let _ = self.add_users_batch_to_users(users).await;

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

    pub async fn add_game_to_games(&self, game: &mut Game) -> Result<Uuid, String> {
        match &self.db_client {
            Some(db_client) => {
                let e = db_client.execute("CREATE TABLE IF NOT EXISTS games (\
                id UUID PRIMARY KEY,
                board_id INT NOT NULL,
                user1_id TEXT,
                user2_id TEXT,
                white_id TEXT,
                black_id TEXT,
                status VARCHAR NOT NULL,
                game_end_condition VARCHAR NOT NULL,
                FOREIGN KEY (board_id) REFERENCES boards (id) ON DELETE CASCADE
                );", &[]).await;

                match game.get_board() {
                    Some(board) => {
                        match self.add_board_to_boards(board).await {
                            Ok(board_id) => {
                                let result = db_client.query_one("
                                INSERT INTO games (id, board_id, user1_id, user2_id, white_id,
                                black_id, status, game_end_condition) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
                                &[
                                    &game.get_game_id(),
                                    &board_id,
                                    &game.get_user1_id(),
                                    &game.get_user2_id(),
                                    &game.get_white_id(),
                                    &game.get_black_id(),
                                    &game.get_game_status(),
                                    &game.get_game_end_condition(),
                                ]).await;

                                match result {
                                    Ok(row) => {
                                        let game_id: Uuid = row.get::<usize, Uuid>(0);
                                        println!("Game created successfully");
                                        Ok(game_id)
                                    },
                                    Err(_) => Err("Could not add game".to_string()),
                                }
                            },
                            Err(e) => Err(e),
                        }
                    },
                    _ => Err("Could not find the board".to_string()),
                }
            },
            _ => Err("Could not connect to the database".to_string()),
        }
    }

    pub async fn get_game_by_id(&self, id: Uuid) -> Result<Game, String> {
        match &self.db_client {
            None => Err("Could not connect to the database".to_string()),
            Some(db_client) => {
                let result = db_client.query_one("\
                SELECT id, board_id, user1_id, user2_id, white_id, black_id, status, game_end_condition
                FROM games WHERE id = $1", &[&id]).await;

                match result {
                    Ok(row) => {
                        let board_id = row.get("board_id");
                        let board = self.get_board_by_id(board_id).await;
                        match board {
                            Ok(board) => {
                                let game = Game::create_game_from_db(
                                    row.get("id"),
                                    board_id,
                                    row.get("user1_id"),
                                    row.get("user2_id"),
                                    row.get("white_id"),
                                    row.get("black_id"),
                                    row.get("status"),
                                    row.get("game_end__condition"),
                                    board,
                                );
                                Ok(game)
                            },
                            _ => Err("Could not get the board".to_string())
                        }
                    },
                    Err(_) => Err("Could not get the board".to_string()),
                }
            }
        }
    }

    pub async fn add_board_to_boards(&self, board: &mut Board) -> Result<i32, String> {
        match &self.db_client {
            Some(db_client) => {
                let e = db_client.execute("
                CREATE TABLE IF NOT EXISTS boards (
                id SERIAL PRIMARY KEY,
                fen TEXT NOT NULL,
                active_color CHAR(1) NOT NULL,
                castle_options TEXT NOT NULL,
                en_passant_square TEXT,
                half_move_clock INT,
                full_move_number INT,
                number_of_columns INT NOT NULL,
                number_of_rows INT NOT NULL,
                columns TEXT NOT NULL,
                rows TEXT NOT NULL
                );"
                ,&[]).await;

                let fen =               board.get_fen();
                let active_color =      board.get_active_color_string();
                let castle_options =    board.get_castle_options();
                let en_passant_square = board.get_en_passant_square();
                let half_move_clock =     board.get_half_move_clock();
                let full_move_number =    board.get_full_move_number();
                let number_of_columns =   board.get_number_of_columns() as i32;
                let number_of_rows =      board.get_number_of_rows() as i32;
                let columns =           board.get_columns();
                let rows =              board.get_rows();

                let query = "
                INSERT INTO boards (fen, active_color, castle_options, en_passant_square, half_move_clock, full_move_number,
                number_of_columns, number_of_rows, columns, rows) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING id";

                let result = db_client.query_one(query,
            &[
                    &fen,
                    &active_color,
                    &castle_options,
                    &en_passant_square,
                    &half_move_clock,
                    &full_move_number,
                    &number_of_columns,
                    &number_of_rows,
                    &columns,
                    &rows,
                ]).await;
                match result {
                    Ok(row) => {
                        let board_id: i32 = row.get::<usize, i32>(0);
                        match self.add_pieces_to_pieces(board_id, board.get_pieces_vec()).await {
                            Ok(_) => {
                                //todo: move board.set_id(board_id) to on_game_added() after its creation
                                board.set_id(board_id);
                                Ok(board_id)
                            },
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => {
                        let a = 1;
                        Err("Could not add board to boards".to_string())
                    },
                }
            },
            _ => Err("Could not connect to the database".to_string()),
        }
    }

    pub async fn add_pieces_to_pieces(&self, board_id: i32, pieces: Vec<PieceEnum>) -> Result<(), String> {
        match &self.db_client {
            None => Err("Could not connect to database".to_string()),
            Some(db_client) => {
                let e = db_client.execute("
                CREATE TABLE IF NOT EXISTS pieces (
                id SERIAL PRIMARY KEY,
                board_id INT NOT NULL,
                coordinates TEXT NOT NULL,
                color TEXT NOT NULL,
                name TEXT NOT NULL,
                symbol TEXT NOT NULL,
                FOREIGN KEY (board_id) REFERENCES boards (id) ON DELETE CASCADE
                );", &[]).await;

                let mut results: Vec<bool> = Vec::new();

                for mut piece in pieces {
                    let result = db_client.query("
                    INSERT INTO pieces (board_id, coordinates, color, name, symbol) VALUES
                    ($1, $2, $3, $4, $5)", &[
                        &board_id,
                        &piece.get_coordinates_string(),
                        &piece.get_color().to_string(),
                        &piece.get_name(),
                        &piece.get_symbol(),
                    ]).await;

                    match result {
                        Ok(_) => results.push(true),
                        _ => results.push(false),
                    }
                }

                match results.iter().all(|&x| x) {
                    true => Ok(()),
                    false => Err("Could not add piece".to_string()),
                }
            }
        }
    }

    pub async fn get_board_by_id(&self, id: i32) -> Result<Board, String> {
        match &self.db_client {
            None => Err("Could not connect to the database".to_string()),
            Some(db_client) => {
                let result = db_client.query_one("\
                SELECT id, fen, active_color, castle_options, en_passant_square,
                half_move_clock, full_move_number, number_of_columns, number_of_rows, columns, rows
                FROM boards WHERE id = $1", &[&id]).await;

                match result {
                    Err(_) => Err("Could not get the board".to_string()),
                    Ok(row) => {
                        let active_color: String = row.get("active_color");
                        let active_color = active_color.chars().nth(0).unwrap();

                        let pieces = self.get_pieces_by_board_id(id.clone()).await;

                        match pieces {
                            Err(e) => Err(e),
                            Ok(pieces) => Ok(
                                Board::new_from_db(
                                    row.get("id"),
                                    row.get("fen"),
                                    pieces,
                                    active_color,
                                    row.get("castle_options"),
                                    row.get("en_passant_square"),
                                    row.get("half_move_clock"),
                                    row.get("full_move_number"),
                                    row.get("number_of_columns"),
                                    row.get("number_of_rows"),
                                    row.get("columns"),
                                    row.get("rows"),
                                )),
                        }
                    }
                }
            },
        }
    }

    pub async fn get_pieces_by_board_id(&self, id: i32) -> Result<HashMap<Coordinates, Option<PieceEnum>>, String>{
        match &self.db_client {
            None => Err("Could not connect to the database".to_string()),
            Some(db_client) => {
                let result = db_client.query("\
                SELECT id, board_id, coordinates, color, name, symbol
                from pieces where board_id = $1", &[&id]).await;
                match result {
                    Err(_) => Err("Could not get pieces".to_string()),
                    Ok(rows) => {
                        let mut pieces: HashMap<Coordinates, Option<PieceEnum>> = HashMap::new();
                        for row in rows {
                            let coordinates: String = row.get("coordinates");
                            let coordinates = Coordinates::new_from_string(&coordinates).unwrap();
                            let color: String = row.get("color");
                            let color = color.chars().nth(0).unwrap();

                            let symbol: String = row.get("symbol");
                            let symbol = symbol.chars().nth(0).unwrap();

                            let piece = PieceEnum::new(
                                coordinates.clone(),
                                symbol,
                            );
                            pieces.insert(coordinates, Some(piece));
                        }
                        Ok(pieces)
                    }
                }
            }
        }
    }

    pub async fn update_game_by_id(&self, game: &Game) -> Result<(), String> {
        match &self.db_client {
            None => Err("Could not connect to the database".to_string()),
            Some(db_client) => {
                let game_id = game.get_game_id().to_string();
                let row_updated = db_client.execute("\
                UPDATE games SET user1_id = $1, user2_id = $2, white_id = $3, black_id = $4, status = $5, game_end_condition = $6 where id = $7
                ", &[
                    &game.get_user1_id(),
                    &game.get_user2_id(),
                    &game.get_white_id(),
                    &game.get_black_id(),
                    &game.get_game_status(),
                    &game.get_game_end_condition(),
                    &game.get_game_id(),
                ]).await;
                match row_updated {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Could not update game by id".to_string()),
                }
            }
        }
    }

    pub fn add_game(
        &mut self,
        game: Game,
        ) -> Result<String, String> {

        self.games_dict.insert(game.get_game_id(), game);
        Ok("Game added successfully".to_string())
    }

    pub fn get_game_by_id_from_dict(&mut self, game_id: Uuid) -> Option<&mut Game> {
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