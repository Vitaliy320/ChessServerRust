use std::collections::HashSet;
use crate::chess_engine::board::Board;
use crate::chess_engine::coordinates::Coordinates;
use crate::chess_engine::piece::Piece;

#[derive(Debug, Clone)]
pub struct Pawn {
    coordinates: Coordinates,
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Pawn {
    pub fn new(color: char, coordinates: Coordinates) -> Pawn {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "P".to_string()
        } else {
            "p".to_string()
        };
        Pawn {
            coordinates,
            color,
            possible_moves: Vec::new(),
            name: String::from("Pawn"),
            symbol,
        }
    }
}

impl AsMut<dyn Piece + 'static> for Pawn {
    fn as_mut(&mut self) -> &mut (dyn Piece + 'static) {
        self
    }
}

impl Piece for Pawn {
    fn get_possible_moves(&self) -> Vec<String> {
        self.possible_moves.clone()
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {
        self.possible_moves = moves
    }

    fn calculate_possible_moves(&mut self, board: &Board) -> Vec<String> {
        if self.color != board.get_active_color().to_char() {
            self.possible_moves = Vec::new();
            return Vec::new();
        }

        let mut possible_moves = HashSet::new();
        let rows = board.get_rows();
        let direction: i8 = if self.color == 'w' { 1 } else { -1 };

        // one square move
        let next_square = Coordinates::new_from_int(
            &self.coordinates.column, &(self.coordinates.row + direction)
        );

        if board.square_is_valid(&next_square)
            && board.square_is_free(&next_square)
            && !board.king_in_check_after_move(&self.coordinates, &next_square) {
            possible_moves.insert(next_square.get_coordinates_string());
        }

        // two squares move from starting position
        if (self.color == 'w' && self.coordinates.row_char == rows.chars().nth(1).unwrap()) ||
            (self.color == 'b' && self.coordinates.row_char == rows.chars().rev().nth(1).unwrap()) {
            let next_square = Coordinates::new_from_int(
                &self.coordinates.column, &(self.coordinates.row + 2 * direction)
            );
            if board.square_is_valid(&next_square)
                && board.square_is_free(&next_square)
                && !board.king_in_check_after_move(&self.coordinates, &next_square) {
                possible_moves.insert(next_square.get_coordinates_string());
            }
        }

        // capture move
        let mut next_square: Coordinates;
        for col_shift in [-1, 1].iter() {
            next_square = Coordinates::new_from_int(
                &(self.coordinates.column + col_shift),
                &(self.coordinates.row + direction)
            );

            if board.square_is_valid(&next_square)
                && board.square_is_capturable(&next_square, &self.color)
                && !board.king_in_check_after_move(&self.coordinates, &next_square) {
                possible_moves.insert(next_square.get_coordinates_string());
            }
        }

        self.possible_moves = possible_moves.into_iter().collect();
        self.possible_moves.clone()
    }

    fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

    fn get_color(&self) -> char {
        self.color.clone()
    }

    fn get_coordinates(&self) -> Coordinates { self.coordinates.clone() }

    fn get_coordinates_string(&self) -> String { self.coordinates.get_coordinates_string() }

    fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    fn set_coordinates_string(&mut self, coordinates: String) {
        if coordinates.len() != 2 {
            return;
        }
        self.coordinates = Coordinates::new_from_string(&coordinates).unwrap();
    }

    fn get_name(&self) -> String { self.name.clone() }
}

unsafe impl Send for Pawn {}
unsafe impl Sync for Pawn {}