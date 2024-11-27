use std::collections::HashSet;
use crate::chess_engine::board::Board;
use crate::chess_engine::color::ActiveColor;
use crate::chess_engine::coordinates::Coordinates;
use crate::chess_engine::piece::Piece;

const QUEEN_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1), (1, 0), (0, -1), (-1, 0),
    (1, 1), (-1, -1), (-1, 1), (1, -1)
];

#[derive(Debug, Clone)]
pub struct Queen {
    coordinates: Coordinates,
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Queen {
    pub fn new(color: char, coordinates: Coordinates) -> Queen {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "Q".to_string()
        } else {
            "q".to_string()
        };
        Queen {
            coordinates,
            color,
            possible_moves: Vec::new(),
            name: String::from("Queen"),
            symbol
        }
    }
}

impl AsMut<dyn Piece + 'static> for Queen {
    fn as_mut(&mut self) -> &mut (dyn Piece + 'static) {
        self
    }
}

impl Piece for Queen {
    fn get_possible_moves(&self) -> Vec<String> {
        self.possible_moves.clone()
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {
        self.possible_moves = moves
    }

    fn calculate_possible_moves(&mut self, board: &Board, color: &ActiveColor, calculate_check_moves: &bool) -> Vec<String> {
        // if self.color != board.get_active_color().to_char() {
        //     self.possible_moves = Vec::new();
        //     return self.get_possible_moves();
        // }

        let mut possible_moves = HashSet::new();
        let mut next_square: Coordinates;

        for direction in QUEEN_DIRECTIONS {
            next_square = self.coordinates.clone();
            loop {
                next_square = Coordinates::new_from_int(
                    &(next_square.column + direction.0),
                    &(next_square.row + direction.1)
                );

                if !board.square_is_valid(&next_square) ||
                    board.square_contains_piece_of_same_color(&next_square, &self.color) {
                    break
                }

                if board.square_is_valid(&next_square)
                    && !(*calculate_check_moves && board.king_in_check_after_move(
                    &self.coordinates,
                    &next_square,
                    &ActiveColor::new_from_char(self.color).unwrap(),
                )) {
                    if board.square_is_free(&next_square) {
                        possible_moves.insert(next_square.to_string());
                    } else if board.square_is_capturable(&next_square, &self.get_color()) {
                        possible_moves.insert(next_square.to_string());
                        break
                    }
                }
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

unsafe impl Send for Queen {}
unsafe impl Sync for Queen {}