use std::collections::HashMap;
use std::ops::DerefMut;
use uuid::Uuid;

use crate::game::Game;

pub struct GameRepository {
    games: HashMap<Uuid, Game>,
}

impl GameRepository {
    pub fn new() -> Self {
        GameRepository {
            games: HashMap::new(),
        }
    }
    pub fn add_game(
        &mut self,
        game_id: Uuid,
        game: Game,
        ) -> Result<String, String> {

        self.games.insert(game_id, game);
        Ok("Game added successfully".to_string())
    }

    pub fn get_game_by_id(&mut self, game_id: Uuid) -> Option<&mut Game> {
        self.games.get_mut(&game_id)
    }
}