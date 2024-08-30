use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use crate::chess_engine::square::Square;
use crate::chess_engine::position::Position;

pub enum Color {
    White,
    Black,
    Random,
}

#[derive(Debug, Clone)]
pub struct Board {
    number_of_columns: u32,
    number_of_rows: u32,
    board_fen: String,
    columns: String,
    rows: String,
    _position: Arc<Mutex<Position>>,
    _board_rows: Vec<Vec<Arc<Mutex<Square>>>>,
}

impl Board {
    pub fn new(columns: String, number_of_columns: u32, rows: String, number_of_rows: u32,
               fen: String) -> Board {
        let position = Arc::new(Mutex::new(Position::new(
            columns.clone(),
            number_of_columns,
            rows.clone(),
            number_of_rows,
            fen.clone()
        )));

        let mut board = Board {
            columns,
            number_of_columns,
            rows,
            number_of_rows,
            board_fen: fen,
            _position: Arc::clone(&position),
            _board_rows: Vec::new(),
        };
        board.initialize_board();
        let cloned_board = board.clone();
        board.update_board(board.clone().board_fen);
        board.calculate_possible_moves();
        board
    }

    fn initialize_board(&mut self) {
        let mut coordinates_string: String;
        let mut column: char;
        let mut row: char;
        let mut squares: HashMap<String, Arc<Mutex<Square>>> = HashMap::new();

        for i in 0..self.number_of_columns {
            column = self.columns.chars().nth(i as usize).unwrap_or('\0');
            for j in 0..self.number_of_rows {
                row = self.rows.chars().nth(j as usize).unwrap_or('\0');

                coordinates_string = column.to_string() + &row.to_string();
                squares.insert(coordinates_string, Arc::new(Mutex::new(Square::new((column, row)))));
                // println!("Coordinates: {:?}", squares.get("a1").unwrap().borrow_mut().get_coordinates())
            }
        }

        self._position.lock().unwrap().set_squares(squares);
        // if let Ok(mut guard) = self._position.write(){
        //     let mut pos = guard.deref_mut();
        //     pos.set_squares(squares);
        // }
        // self._position.borrow_mut().set_squares(squares);
    }

    pub fn get_position(&self) -> Arc<Mutex<Position>> {
        self._position.clone()
    }

    pub fn get_columns(&self) -> String {
        self.columns.clone()
    }

    pub fn get_rows(&self) -> String {
        self.rows.clone()
    }

    pub fn update_board(&mut self, fen: String) {
        self.calculate_possible_moves();
        self._position.lock().unwrap().position_from_fen(fen);
    }

    pub fn make_move(&mut self, move_from: (char, char), move_to: (char, char)) {
        self.calculate_possible_moves();
        self._position.lock().unwrap().make_move(move_from, move_to);
    }

    pub fn get_square_by_coordinates(&self, coordinates: (char, char)) -> Arc<Mutex<Square>> {
        let pos = self._position.lock().unwrap();
        pos.get_square_by_coordinates(coordinates)
    }

    pub fn make_move_str(&mut self, move_from: String, move_to: String) {
        self.calculate_possible_moves();
        let mut pos = self._position.lock().unwrap();
        pos.make_move(
            (move_from.chars().nth(0).unwrap(), move_from.chars().nth(1).unwrap()),
            (move_to.chars().nth(0).unwrap(), move_to.chars().nth(1).unwrap())
        );
        let s = pos.get_square_by_coordinates(('e', '2'))
            .lock()
            .unwrap()
            .square_to_str();
        println!("make_move_str square: {}", s);
    }

    pub fn create_board_rows(&mut self) {
        let mut current_row: Vec<Arc<Mutex<Square>>>;
        let mut all_rows: Vec<Vec<Arc<Mutex<Square>>>> = Vec::new();

        for j in (0..self.number_of_rows).rev() {
            current_row = Vec::new();
            if let Ok(mut guard) = self._position.lock() {
                let mut pos = guard.deref_mut();
                for i in 0..self.number_of_columns {
                    current_row.push(
                        pos.get_square_by_coordinates(
                            (self.columns.chars().nth(i as usize).unwrap(),
                            self.rows.chars().nth(j as usize).unwrap()))

                    );
                }
            }
            all_rows.push(current_row);
        }

        self._board_rows = all_rows;
    }

    fn calculate_possible_moves(&mut self) {
        let position = self.get_position();
        let mut position = position.lock().unwrap();

        position.get_squares().values()
            .into_iter()
            .for_each(|square| {
                let mut square = square.lock().unwrap();
                let piece_mut_ref = square.get_piece_mut();

                // Match statement to handle different cases
                match piece_mut_ref.as_mut() {
                    Some(piece) => {
                        piece.lock().unwrap().calculate_possible_moves();
                    },
                    _ => (),
                }
            });
    }

    pub fn board_to_string(&mut self) -> String {
        self.create_board_rows();

        let mut rows_vector: Vec<String> = Vec::new();

        let mut row_str: String = "".to_string();
        let mut rows_str = "                  \n                  ".to_string();
        rows_vector.push("                  ".to_string());
        rows_vector.push("                  ".to_string());

        let mut row: &Vec<Arc<Mutex<Square>>> = &Vec::new();

        for i in 0..self._board_rows.len() {
            // row_str = "".to_string();
            let mut current_row: String = "".to_string();
            let val = (8 - i).to_string() + &' '.to_string();
            current_row.push_str(val.as_str());
            // row_str += &val;

            row = &self._board_rows[i];
            for mut square in row {
                if let Ok(mut guard) = square.lock() {
                    let sq = guard.deref_mut();
                    match sq.get_piece() {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
