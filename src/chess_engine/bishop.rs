use std::collections::HashSet;
use crate::chess_engine::board::Board;
use crate::chess_engine::coordinates::Coordinates;
use crate::chess_engine::piece::Piece;

#[derive(Debug, Clone)]
pub struct Bishop {
    coordinates: Coordinates,
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Bishop {
    pub fn new(color: char, coordinates: Coordinates) -> Bishop {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "B".to_string()
        } else {
            "b".to_string()
        };
        Bishop {
            coordinates,
            color,
            possible_moves: Vec::new(),
            name: String::from("Bishop"),
            symbol
        }
    }
}

impl AsMut<dyn Piece + 'static> for Bishop {
    fn as_mut(&mut self) -> &mut (dyn Piece + 'static) {
        self
    }
}

impl Piece for Bishop {
    fn get_possible_moves(&self) -> Vec<String> {
        self.possible_moves.clone()
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {
        self.possible_moves = moves
    }

    fn calculate_possible_moves(&mut self, board: &Board) -> Vec<String> {
        if self.color != board.get_active_color().to_char() {
            self.possible_moves = Vec::new();
            return self.get_possible_moves();
        }

        let mut possible_moves = HashSet::new();
        let mut next_square: Coordinates;
        let directions = [
            (1, 1), (-1, -1), (-1, 1), (1, -1)
        ];

        for direction in directions {
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
                    && !board.king_in_check_after_move(&self.coordinates, &next_square) {
                    if board.square_is_free(&next_square) {
                        possible_moves.insert(next_square.get_coordinates_string());
                    } else if board.square_is_capturable(&next_square, &self.get_color()) {
                        possible_moves.insert(next_square.get_coordinates_string());
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

unsafe impl Send for Bishop {}
unsafe impl Sync for Bishop {}