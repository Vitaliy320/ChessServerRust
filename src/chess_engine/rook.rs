use crate::chess_engine::piece::Piece;

#[derive(Debug)]
pub struct Rook {
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Rook {
    pub fn new(color: char) -> Rook {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "R".to_string()
        } else {
            "r".to_string()
        };
        Rook {color,
            possible_moves: Vec::new(),
            name: String::from("Rook"),
            symbol,
        }
    }
}

impl Piece for Rook {
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

unsafe impl Send for Rook {}
unsafe impl Sync for Rook {}