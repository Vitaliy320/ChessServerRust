use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::game_status::GameStatus;
use crate::chess_engine::board::Board;

#[derive(Clone, Debug)]
pub struct Game {
    game_id: Uuid,
    board_id: Option<i32>,
    user1_id: Option<String>,
    user2_id: Option<String>,
    white_id: Option<String>,
    black_id: Option<String>,
    status: GameStatus,
    board: Option<Board>,
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

        //todo: replace uuid with i32. When the board is created, the board_id field will be updated
        let game_id = Uuid::new_v4();

        let board = Board::create_board_from_fen(columns, board_size, rows, board_size, board_fen);

        let mut game = Game {
            game_id,
            user1_id: From::from(user_id),
            user2_id: None,
            white_id,
            black_id,
            status: GameStatus::AwaitingOpponent,
            board_id: None,
            board: Some(board),
        };

        game
    }

    pub fn create_game_from_board(user_id: String, board: Board, color: String) -> Game {
        let mut game = Game::new(user_id, color);
        game.set_board(board);
        game
    }

    pub fn create_game_from_db(
        game_id: Uuid,
        board_id: i32,
        user1_id: Option<String>,
        user2_id: Option<String>,
        white_id: Option<String>,
        black_id: Option<String>,
        status: GameStatus,
        board: Board) -> Game {
        Game {
            game_id,
            board_id: Some(board_id),
            user1_id,
            user2_id,
            white_id,
            black_id,
            board: Some(board),
            status,
        }
    }

    pub fn get_board(&mut self) -> Option<&mut Board> {
        self.board.as_mut()
    }
    pub fn set_board(&mut self, board: Board) {
        self.board = Some(board);
    }

    pub fn get_board_id(&self) -> Option<i32> {
        self.board_id
    }

    pub fn set_user(&mut self, user1_id: Option<String>, user2_id: Option<String>) {
        // todo: refactor
        match user1_id {
            Some(user_id) => self.user1_id = Some(user_id),
            None => (),
        }
        match user2_id {
            Some(user_id) => {
                self.user2_id = Some(user_id.clone());
                match (&self.white_id, &self.black_id) {
                    (None, _) => self.white_id = Some(user_id.clone()),
                    (_, None) => self.black_id = Some(user_id.clone()),
                    _ => (),
                }
                self.status = GameStatus::Ongoing;
            },
            None => (),
        }
    }

    pub fn get_user1_id(&self) -> Option<String> {
        self.user1_id.clone()
    }

    pub fn get_user2_id(&self) -> Option<String> {
        self.user2_id.clone()
    }

    pub fn get_white_id(&self) -> Option<String> {
        self.white_id.clone()
    }

    pub fn get_black_id(&self) -> Option<String> {
        self.black_id.clone()
    }

    pub fn get_game_id(&self) -> Uuid {
        self.game_id.clone()
    }

    pub fn get_game_status(&self) -> GameStatus {
        self.status.clone()
    }

    pub fn get_users(&self) -> (Option<String>, Option<String>) {
        (self.user1_id.clone(), self.user2_id.clone())
    }
}