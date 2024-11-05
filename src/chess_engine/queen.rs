use crate::chess_engine::board::Board;
use crate::chess_engine::coordinates::Coordinates;
use crate::chess_engine::piece::Piece;

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

    fn calculate_possible_moves(&mut self, board: &Board) -> Vec<String> {
        Vec::new()
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

unsafe impl Send for Queen {}
unsafe impl Sync for Queen {}