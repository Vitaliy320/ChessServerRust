use crate::chess_engine::piece::Piece;

#[derive(Debug, Clone)]
pub struct Knight {
    coordinates: (char, char),
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Knight {
    pub fn new(color: char, coordinates: (char, char)) -> Knight {
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

    // fn as_mut(self: Box<Knight>) -> Box<Knight> {
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

unsafe impl Send for Knight {}
unsafe impl Sync for Knight {}