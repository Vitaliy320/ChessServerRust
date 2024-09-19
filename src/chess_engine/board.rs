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

pub enum Color {
    White,
    Black,
    Random,
}

#[derive(Debug, Clone)]
pub struct Board {
    fen: String,
    squares_dict: HashMap<String, Square>,
    active_color: char,
    castle_options: String,
    en_passant_square: String,
    half_move_clock: Option<i32>,
    full_move_number: Option<i32>,
    number_of_columns: u32,
    number_of_rows: u32,
    columns: String,
    rows: String,
    pieces: HashMap<char, (TypeId, char)>,
    squares_vec: Vec<Vec<Square>>,
}

impl Board {
    pub fn new(
        columns: String,
        n_of_columns: u32,
        rows: String,
        n_of_rows: u32,
        fen: String,
    ) -> Board {
        let mut board = Board {
            fen: fen.clone(),
            number_of_columns: n_of_columns,
            number_of_rows: n_of_rows,
            columns,
            rows,
            squares_dict: HashMap::new(),
            active_color: 'w',
            castle_options: "".to_string(),
            en_passant_square: "".to_string(),
            half_move_clock: None,
            full_move_number: None,
            pieces: HashMap::new(),
            squares_vec: Vec::new(),
        };
        board.initialize_board();
        board.board_from_fen(fen);
        board.calculate_possible_moves();
        board
    }

    fn initialize_board(&mut self) {
        let mut coordinates_string: String;
        let mut column: char;
        let mut row: char;
        let mut squares: HashMap<String, Square> = HashMap::new();

        for i in 0..self.number_of_columns {
            column = self.columns.chars().nth(i as usize).unwrap_or('\0');
            for j in 0..self.number_of_rows {
                row = self.rows.chars().nth(j as usize).unwrap_or('\0');

                coordinates_string = column.to_string() + &row.to_string();
                squares.insert(coordinates_string, Square::new((column, row)));
                // println!("Coordinates: {:?}", squares.get("a1").unwrap().borrow_mut().get_coordinates())
            }
        }

        self.squares_dict = squares;
    }

    pub fn set_squares(&mut self, squares: HashMap<String, Square>) {
        self.squares_dict = squares;
    }

    pub fn get_squares(&mut self) -> &mut HashMap<String, Square> {
        &mut self.squares_dict
    }

    pub fn get_columns(&self) -> String {
        self.columns.clone()
    }

    pub fn get_rows(&self) -> String {
        self.rows.clone()
    }

    fn _create_piece(&self, piece_symbol: char) -> Arc<Mutex<dyn Piece + Send + Sync>> {
        //todo: replace Arc<Mutex<>> with owned values
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

    pub fn board_from_fen(&mut self, fen: String) {
        let split_fen: Vec<String>        = fen.split(' ').map(|s| String::from(s)).collect();
        let board_fen             = &split_fen[0];
        let color_fen             = &split_fen[1];
        let castle_fen            = &split_fen[2];
        let en_passant_fen        = &split_fen[3];
        let half_move_fen         = &split_fen[4];
        let full_move_number_fen  = &split_fen[5];

        self.active_color = color_fen.chars().nth(0).unwrap();
        self.castle_options = castle_fen.clone();
        self.en_passant_square = en_passant_fen.clone();
        self.half_move_clock = half_move_fen.parse().ok();
        self.full_move_number = full_move_number_fen.parse().ok();

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
            row_coordinate = self.rows.chars().nth(j).unwrap();
            column_counter = 0;

            for cell_value in row.chars() {
                if cell_value.is_digit(10) {
                    column_counter += cell_value as i32 - '0' as i32;
                } else {
                    piece = self._create_piece(cell_value);
                    column_coordinate = self.columns.chars()
                        .nth(column_counter as usize)
                        .unwrap();

                    let mut square = self.get_square_by_coordinates(
                        (column_coordinate.clone(),
                         row_coordinate.clone()));
                    square.set_piece(piece);
                    column_counter += 1;
                }
            }
        }
    }

    pub fn get_square_by_coordinates(&mut self, coordinates: (char, char)) -> &mut Square {
        let key: String = coordinates.0.to_string() + &coordinates.1.to_string();
        self.squares_dict.get_mut(&key).unwrap()
    }

    pub fn make_move(&mut self, move_from: (char, char), move_to: (char, char)) {
        self.calculate_possible_moves();
        let mut piece = {
            let mut from_square: &mut Square = self.get_square_by_coordinates(move_from);
            let mut piece = from_square.get_piece_mut().clone();
            from_square.remove_piece();
            piece
        };
        if let Some(piece) = piece.as_mut() {
            let mut to_square: &mut Square = self.get_square_by_coordinates(move_to);
            to_square.set_piece(Arc::clone(piece));
        }
    }

    pub fn make_move_str(&mut self, move_from: String, move_to: String) {
        self.make_move(
            (move_from.chars().nth(0).unwrap(), move_from.chars().nth(1).unwrap()),
            (move_to.chars().nth(0).unwrap(), move_to.chars().nth(1).unwrap())
        );
        let s = self.get_square_by_coordinates(('e', '2'))
            .square_to_str();
        println!("make_move_str square: {}", s);
    }

    fn calculate_possible_moves(&mut self) {
        self.get_squares().values_mut()
            .into_iter()
            .for_each(|mut square| {
                let piece_mut_ref = square.get_piece_mut();

                match piece_mut_ref.as_mut() {
                    Some(piece) => {
                        piece.lock().unwrap().calculate_possible_moves();
                    },
                    _ => (),
                }
            });
    }

    pub fn create_squares_vec(&mut self) {
        let mut current_row: Vec<Square>;
        let mut all_rows: Vec<Vec<Square>> = Vec::new();

        for j in (0..self.number_of_rows).rev() {
            current_row = Vec::new();
            for i in 0..self.number_of_columns {
                current_row.push(
                    self.get_square_by_coordinates((
                        self.columns.chars().nth(i as usize).unwrap(),
                        self.rows.chars().nth(j as usize).unwrap()
                    )).clone()
                );
            }
            all_rows.push(current_row);
        }
        self.squares_vec = all_rows;
    }

    pub fn board_to_string(&mut self) -> String {
        self.create_squares_vec();

        let mut rows_vector: Vec<String> = Vec::new();

        let mut row_str: String = "".to_string();
        let mut rows_str = "                  \n                  ".to_string();
        rows_vector.push("                  ".to_string());
        rows_vector.push("                  ".to_string());

        let mut row: &Vec<Square> = &Vec::new();

        for i in 0..self.squares_vec.len() {
            // row_str = "".to_string();
            let mut current_row: String = "".to_string();
            let val = (8 - i).to_string() + &' '.to_string();
            current_row.push_str(val.as_str());
            // row_str += &val;

            row = &self.squares_vec[i];
            for mut square in row {
                match square.get_piece() {
                    Some(piece) => {
                        current_row += &piece.lock().unwrap().get_symbol();
                        current_row += " ";
                        row_str += &piece.lock().unwrap().get_symbol();
                        row_str += " ";
                    },
                    None => {
                        current_row.push_str("  ");
                        row_str += " ";
                    },
                }
            }
            rows_str.push_str("\n");
            rows_str.push_str(current_row.as_str());
            // rows_str = "\n".to_owned() + row_str.as_str();
            rows_vector.push(current_row);
            // rows_str += row_str.as_str();
        }

        rows_vector.push("  A B C D E F G H ".to_string());
        rows_str += "\n  A B C D E F G H ";
        rows_str
    }
}

