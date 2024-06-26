use std::sync::{Arc, RwLock};
use chesslib::board::Board;

pub struct Game {
    _board: Option<Board>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            _board: None,
        }
    }

    pub fn create_game_from_board(board: Board) -> Game {
        let mut game = Game::new();
        game.set_board(board);
        game
    }
    pub fn get_board(&mut self) -> Option<&mut Board> {
        self._board.as_mut()
    }
    pub fn set_board(&mut self, board: Board) {
        self._board = Some(board);
    }
}