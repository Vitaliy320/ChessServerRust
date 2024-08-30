use std::collections::HashMap;
use std::any::TypeId;
use std::sync::RwLock;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};

use crate::chess_engine::square::Square;
use crate::chess_engine::pawn::Pawn;
use crate::chess_engine::knight::Knight;
use crate::chess_engine::bishop::Bishop;
use crate::chess_engine::rook::Rook;
use crate::chess_engine::queen::Queen;
use crate::chess_engine::king::King;
use crate::chess_engine::piece::{Piece, DefaultPiece};

#[derive(Debug)]
pub struct Position {
    _fen: String,
    _squares: HashMap<String, Arc<Mutex<Square>>>,
    _active_color: char,
    _castle_options: String,
    _en_passant_square: String,
    _half_move_clock: Option<i32>,
    _full_move_number: Option<i32>,
    _number_of_columns: u32,
    _number_of_rows: u32,
    _columns: String,
    _rows: String,
    _pieces: HashMap<char, (TypeId, char)>,
}

impl Position {
    pub fn new(
        columns: String,
        n_of_columns: u32,
        rows: String,
        n_of_rows: u32,
        fen: String,
    ) -> Position {
        Position {
            _fen: fen,
            _number_of_columns: n_of_columns,
            _number_of_rows: n_of_rows,
            _columns: columns,
            _rows: rows,
            _squares: HashMap::new(),
            _active_color: 'w',
            _castle_options: "".to_string(),
            _en_passant_square: "".to_string(),
            _half_move_clock: None,
            _full_move_number: None,
            _pieces: HashMap::new(),
        }
    }

    pub fn set_squares(&mut self, squares: HashMap<String, Arc<Mutex<Square>>>) {
        self._squares = squares;
    }

    pub fn get_squares(&self) -> &HashMap<String, Arc<Mutex<Square>>> {
        &self._squares
    }

    fn _create_piece(&self, piece_symbol: char) -> Arc<Mutex<dyn Piece + Send + Sync>> {
        match piece_symbol {
            'p' => Arc::new(Mutex::new(Pawn::new('b'))),
            'n' => Arc::new(Mutex::new(Knight::new('b'))),
            'b' => Arc::new(Mutex::new(Bishop::new('b'))),
            'r' => Arc::new(Mutex::new(Rook::new('b'))),
            'q' => Arc::new(Mutex::new(Queen::new('b'))),
            'k' => Arc::new(Mutex::new(King::new('b'))),

            'P' => Arc::new(Mutex::new(Pawn::new('w'))),
            'N' => Arc::new(Mutex::new(Knight::new('w'))),
            'B' => Arc::new(Mutex::new(Bishop::new('w'))),
            'R' => Arc::new(Mutex::new(Rook::new('w'))),
            'Q' => Arc::new(Mutex::new(Queen::new('w'))),
            'K' => Arc::new(Mutex::new(King::new('w'))),
            _ => {Arc::new(Mutex::new(DefaultPiece::new()))},
        }
    }

    pub fn position_from_fen(&mut self, fen: String) {
        let split_fen: Vec<String>        = fen.split(' ').map(|s| String::from(s)).collect();
        let board_fen             = &split_fen[0];
        let color_fen             = &split_fen[1];
        let castle_fen            = &split_fen[2];
        let en_passant_fen        = &split_fen[3];
        let half_move_fen         = &split_fen[4];
        let full_move_number_fen  = &split_fen[5];

        self._active_color = color_fen.chars().nth(0).unwrap();
        self._castle_options = castle_fen.clone();
        self._en_passant_square = en_passant_fen.clone();
        self._half_move_clock = half_move_fen.parse().ok();
        self._full_move_number = full_move_number_fen.parse().ok();

        let mut board_rows: Vec<String> = board_fen.split('/')
            .map(|s| String::from(s)).collect();
        board_rows.reverse();

        let mut row: String;
        let mut piece: Arc<Mutex<dyn Piece>>;
        let mut column_counter: i32;
        let mut column_coordinate: char;
        let mut row_coordinate: char;

        for j in 0..board_rows.len() {
            row = board_rows[j].clone();
            row_coordinate = self._rows.chars().nth(j).unwrap();
            column_counter = 0;

            for cell_value in row.chars() {
                if cell_value.is_digit(10) {
                    column_counter += cell_value as i32 - '0' as i32;
                } else {
                    piece = self._create_piece(cell_value);
                    column_coordinate = self._columns.chars()
                                                     .nth(column_counter as usize)
                                                     .unwrap();

                    let mut square = self.get_square_by_coordinates(
                        (column_coordinate.clone(),
                                   row_coordinate.clone()));
                    let mut square = square.lock().unwrap();
                    square.set_piece(piece);
                    column_counter += 1;
                }
            }
        }
    }

    pub fn get_square_by_coordinates(&self, coordinates: (char, char)) -> Arc<Mutex<Square>> {
        let key: String = coordinates.0.to_string() + &coordinates.1.to_string();
        self._squares.get(&key).unwrap().clone()
    }

    pub fn make_move(&self, move_from: (char, char), move_to: (char, char)) {
        let mut from_square = self.get_square_by_coordinates(move_from);
        let mut to_square = self.get_square_by_coordinates(move_to);
        if let (Ok(mut from), Ok(mut to)) = (from_square.lock(), to_square.lock()) {
            let mut piece = from.get_piece_mut().clone();
            if let Some(piece) = piece.as_mut() {
                from.remove_piece();
                to.set_piece(Arc::clone(piece));
            }
        };
    }
}

