use std::collections::HashMap;
use std::ops::DerefMut;
use uuid::Uuid;

use crate::game::Game;
use crate::game::GameStatus;

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