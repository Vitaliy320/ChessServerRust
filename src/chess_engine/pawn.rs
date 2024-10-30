use crate::chess_engine::piece_new::Piece;

#[derive(Debug, Clone)]
pub struct Pawn {
    coordinates: (char, char),
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Pawn {
    pub fn new(color: char, coordinates: (char, char)) -> Pawn {
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
            symbol
        }
    }
}

impl AsMut<dyn Piece + 'static> for Pawn {
    fn as_mut(&mut self) -> &mut (dyn Piece + 'static) {
        self
    }
}

impl Piece for Pawn {
    fn get_possible_moves(&self) -> &Vec<String> {
        &self.possible_moves
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {
        self.possible_moves = moves
    }

    fn calculate_possible_moves(&mut self) {
        self.possible_moves = Vec::new()
    }

    fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

    fn get_color(&self) -> char {
        self.color.clone()
    }

    // fn as_mut(self: Box<Pawn>) -> Box<Pawn> {
    //     self
    // }

    fn get_coordinates_string(&self) -> String { format!("{}{}", self.coordinates.0, self.coordinates.1) }

    fn set_coordinates(&mut self, coordinates: (char, char)) {
        self.coordinates = coordinates;
    }

    fn set_coordinates_string(&mut self, coordinates: String) {
        if coordinates.len() != 2 {
            return;
        }
        self.coordinates = (coordinates.chars().nth(0).unwrap(),
                            coordinates.chars().nth(1).unwrap());
    }

    fn get_name(&self) -> String { self.name.clone() }
}

unsafe impl Send for Pawn {}
unsafe impl Sync for Pawn {}