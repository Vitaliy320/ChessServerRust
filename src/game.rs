use std::sync::{Arc, RwLock};
use uuid::Uuid;

use chesslib::board::Board;
#[derive(Clone)]
enum GameStatus {
    Awaiting,
    Ongoing,
    Finished,
}
#[derive(Clone)]
pub struct Game {
    _game_id: Uuid,
    _user1_id: Option<String>,
    _user2_id: Option<String>,
    _white_id: Option<String>,
    _black_id: Option<String>,
    _status: GameStatus,
    _board: Option<Board>,
}

impl Game {
    pub fn new(game_id: Uuid, user_id: String, color: String) -> Game {
        let (white_id, black_id) = match color.as_str() {
            "white" => (From::from(user_id.clone()), None),
            "black" => (None, From::from(user_id.clone())),
            _ => (From::from(user_id.clone()), None),
        };

        let columns = "abcdefgh".to_string();
        let rows = "12345678".to_string();
        let board_size = 8;
        let board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

        let mut game = Game {
            _game_id: game_id,
            _user1_id: From::from(user_id),
            _user2_id: None,
            _white_id: white_id,
            _black_id: black_id,
            _status: GameStatus::Ongoing,
            _board: From::from(Board::new(columns, board_size, rows, board_size, board_fen)),
        };

        game
    }

    pub fn create_game_from_board(game_id: Uuid, user_id: String, board: Board, color: String) -> Game {
        let mut game = Game::new(game_id, user_id, color);
        game.set_board(board);
        game
    }
    pub fn get_board(&mut self) -> Option<&mut Board> {
        self._board.as_mut()
    }
    pub fn set_board(&mut self, board: Board) {
        self._board = Some(board);
    }

    pub fn set_user(&mut self, user1_id: Option<String>, user2_id: Option<String>) {
        match user1_id {
            Some(user_id) => self._user1_id = Some(user_id),
            None => (),
        }
        match user2_id {
            Some(user_id) => self._user2_id = Some(user_id),
            None => (),
        }
    }
    pub fn get_game_id(&self) -> Uuid {
        self._game_id.clone()
    }
}