use std::sync::{Arc, RwLock};
use uuid::Uuid;

use chesslib::board::Board;
#[derive(Clone, Debug)]
pub enum GameStatus {
    AwaitingOpponent,
    Ongoing,
    Finished,
}
#[derive(Clone, Debug)]
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
    pub fn new(user_id: String, color: String) -> Game {
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
            _game_id: Uuid::new_v4(),
            _user1_id: From::from(user_id),
            _user2_id: None,
            _white_id: white_id,
            _black_id: black_id,
            _status: GameStatus::AwaitingOpponent,
            _board: From::from(Board::new(columns, board_size, rows, board_size, board_fen)),
        };

        game
    }

    pub fn create_game_from_board(user_id: String, board: Board, color: String) -> Game {
        let mut game = Game::new(user_id, color);
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
            Some(user_id) => {
                self._user2_id = Some(user_id.clone());
                match (&self._white_id, &self._black_id) {
                    (None, _) => self._white_id = Some(user_id.clone()),
                    (_, None) => self._black_id = Some(user_id.clone()),
                    _ => (),
                }
                self._status = GameStatus::Ongoing;
            },
            None => (),
        }
    }
    pub fn get_game_id(&self) -> Uuid {
        self._game_id.clone()
    }

    pub fn get_game_status(&self) -> GameStatus {
        self._status.clone()
    }

    pub fn get_users(&self) -> (Option<String>, Option<String>) {
        (self._user1_id.clone(), self._user2_id.clone())
    }
}