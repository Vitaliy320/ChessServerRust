use std::collections::HashMap;
use uuid::Uuid;

use crate::game_status::GameStatus;
use crate::chess_engine::board::Board;
use crate::game_end_condition::GameEndCondition;
use crate::chess_engine::color::ActiveColor;

#[derive(Clone, Debug)]
pub struct Game {
    game_id: Uuid,
    board_id: Option<i32>,
    user1_id: Option<String>,
    user2_id: Option<String>,
    white_id: Option<String>,
    black_id: Option<String>,
    pub color_by_user_id: HashMap<String, ActiveColor>,
    status: GameStatus,
    game_end_condition: GameEndCondition,
    board: Option<Board>,
}

impl Game {
    pub fn new(user_id: String, color: String) -> Game {
        let mut color_by_user_id = HashMap::new();
        let (white_id, black_id) = match color.as_str() {
            "white" => {
                color_by_user_id.insert(user_id.clone(), ActiveColor::White);
                (Some(user_id.clone()), None)
            },
            "black" => {
                color_by_user_id.insert(user_id.clone(), ActiveColor::Black);
                (None, Some(user_id.clone()))
            },
            _ => {
                color_by_user_id.insert(user_id.clone(), ActiveColor::White);
                (Some(user_id.clone()), None)
            },
        };

        let columns = "abcdefgh".to_string();
        let rows = "12345678".to_string();
        let board_size = 8;
        // let board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let board_fen = "rnb2bnr/1ppkpppp/p7/3N4/8/8/PPPP1PPP/RNBQKB1R w KQkq - 0 1".to_string();

        //todo: replace uuid with i32. When the board is created, the board_id field will be updated
        let game_id = Uuid::new_v4();

        let board = Board::new_from_fen(columns, board_size, rows, board_size, board_fen);

        let mut game = Game {
            game_id,
            user1_id: From::from(user_id),
            user2_id: None,
            white_id,
            black_id,
            color_by_user_id,
            status: GameStatus::AwaitingOpponent,
            game_end_condition: GameEndCondition::None,
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
        game_end_condition: GameEndCondition,
        board: Board
    ) -> Game {
        let mut color_by_user_id = HashMap::new();

        if let Some(white_id) = &white_id {
            color_by_user_id.insert(white_id.clone(), ActiveColor::White);
        }

        if let Some(black_id) = &black_id {
            color_by_user_id.insert(black_id.clone(), ActiveColor::Black);
        }

        Game {
            game_id,
            board_id: Some(board_id),
            user1_id,
            user2_id,
            white_id,
            black_id,
            color_by_user_id,
            board: Some(board),
            status,
            game_end_condition,
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
                    (None, _) => {
                        self.color_by_user_id.insert(user_id.clone(), ActiveColor::White);
                        self.white_id = Some(user_id.clone())
                    },
                    (_, None) => self.black_id = {
                        self.color_by_user_id.insert(user_id.clone(), ActiveColor::Black);
                        Some(user_id.clone())
                    },
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
    pub fn get_game_end_condition(&self) -> GameEndCondition {
        self.game_end_condition.clone()
    }

    pub fn get_users(&self) -> (Option<String>, Option<String>) {
        (self.user1_id.clone(), self.user2_id.clone())
    }
}