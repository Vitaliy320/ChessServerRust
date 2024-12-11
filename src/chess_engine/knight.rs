use std::collections::HashSet;
use crate::chess_engine::board::Board;
use crate::chess_engine::color::ActiveColor;
use crate::chess_engine::coordinates::Coordinates;
use crate::chess_engine::piece::Piece;

const KNIGHT_DIRECTIONS: [(i8, i8); 8] = [
    (1, 2), (2, 1), (2, -1), (1, -2),
    (-1, -2), (-2, -1), (-2, 1), (-1, 2),
];

#[derive(Debug, Clone)]
pub struct Knight {
    coordinates: Coordinates,
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Knight {
    pub fn new(color: char, coordinates: Coordinates) -> Knight {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "N".to_string()
        } else {
            "n".to_string()
        };
        Knight {
            coordinates,
            color,
            possible_moves: Vec::new(),
            name: String::from("Knight"),
            symbol
        }
    }
}

impl AsMut<dyn Piece + 'static> for Knight {
    fn as_mut(&mut self) -> &mut (dyn Piece + 'static) {
        self
    }
}

impl Piece for Knight {
    fn get_possible_moves(&self) -> Vec<String> {
        self.possible_moves.clone()
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {
        self.possible_moves = moves
    }

    fn generate_piece_moves(&mut self, board: &Board, color: &ActiveColor, calculate_check_moves: &bool) -> Vec<String> {
        // if self.color != board.get_active_color().to_char() {
        //     self.possible_moves = Vec::new();
        //     return self.get_possible_moves();
        // }

        let mut possible_moves = HashSet::new();
        let mut next_square: Coordinates;

        for direction in KNIGHT_DIRECTIONS {
            next_square = Coordinates::new_from_int(
                &(self.coordinates.column + direction.0),
                &(self.coordinates.row + direction.1)
            );

            if board.square_is_valid(&next_square) &&
                !board.square_contains_piece_of_same_color(&next_square, &self.color)
                && !(*calculate_check_moves && board.king_in_check_after_move(
                &self.coordinates,
                &next_square,
                &ActiveColor::new_from_char(self.color).unwrap(),
            )) {
                possible_moves.insert(next_square.to_string());
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

    fn get_coordinates_string(&self) -> String { self.coordinates.to_string() }

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

unsafe impl Send for Knight {}
unsafe impl Sync for Knight {}