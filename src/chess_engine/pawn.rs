use crate::chess_engine::piece::Piece;

#[derive(Debug)]
pub struct Pawn {
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Pawn {
    pub fn new(color: char) -> Pawn {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "P".to_string()
        } else {
            "p".to_string()
        };
        Pawn {color: color,
            possible_moves: Vec::new(),
            name: String::from("Pawn"),
            symbol: symbol
        }
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
}

unsafe impl Send for Pawn {}
unsafe impl Sync for Pawn {}