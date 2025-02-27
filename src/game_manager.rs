use std::collections::HashMap;
use std::ops::Deref;
use uuid::Uuid;
use crate::connection_manager::ConnectionManager;
use crate::game::Game;
use crate::game_repository::GameRepository;
use crate::game_status::GameStatus;

pub struct GameManager {
    pub game_repository: GameRepository,
    games: HashMap<Uuid, Box<Game>>,
    pub connection_manager: ConnectionManager,
}

impl GameManager {
    pub fn new(game_repository: GameRepository) -> GameManager {
        GameManager {
            game_repository,
            games: HashMap::new(),
            connection_manager: ConnectionManager::new(),
        }
    }

    pub async fn add_game_to_games(&mut self, mut game: Game) -> Result<(Uuid, i32), String> {
        let result = self.game_repository.add_game_to_games(&mut game).await;
        match result {
            Ok((game_id, board_id)) => {
                game.get_board_mut().set_id(board_id);
                game.set_board_id(board_id);
                self.games.insert(game_id, Box::new(game));
                result
            },
            Err(e) => Err(e),
        }

    }

    pub fn get_awaiting_games(&self) -> Vec<Uuid> {
        let ids: Vec<Uuid> = self.games.clone().iter()
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

    pub async fn get_game_by_id(&self, game_id: &Uuid) -> Result<&Game, String> {
        match self.games.get(game_id) {
            Some(game) => Ok(game),
            _ => Err("Could not find a game".to_string()),
        }
    }

    pub async fn get_mutable_game_by_id(&mut self, game_id: &Uuid) -> Result<&mut Game, String> {
        match self.games.get_mut(game_id) {
            Some(game) => Ok(game),
            _ => Err("Could not find a game".to_string()),
        }
    }

    pub async fn update_game_by_id(&mut self, game_id: &Uuid) -> Result<(), String> {
        match self.games.get_mut(game_id) {
            Some(game) => {
                *game = game.clone();
                match self.game_repository.update_game_by_id_db(game).await {
                    Ok(_) => {
                        let board_id = game.get_board_id().unwrap();
                        self.game_repository.update_board_by_id(board_id, game.get_board()).await
                    },
                    Err(e) => Err(e),
                }

            },
            _ => Err("Could not find a game".to_string()),
        }
    }

    // pub async fn update_board_by_game_id(&self, game_id: &Uuid) -> Result<(), String> {
        // match self.games.get(game_id) {
        //     Some(mut game) => {
        //         match game.get_board_mut() {
        //             Some(mut board) => {
        //                 game.set_board(board.clone());
        //
        //             }
        //         }
        //     }
        // }
        // self.game_repository.update_board_by_id(board_id).await
    // }
}