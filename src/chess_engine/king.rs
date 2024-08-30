use crate::chess_engine::piece::Piece;

#[derive(Debug)]
pub struct King {
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl King {
    pub fn new(color: char) -> King {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "K".to_string()
        } else {
            "k".to_string()
        };
        King {color: color,
            possible_moves: Vec::new(),
            name: String::from("King"),
            symbol: symbol
        }
    }
}

impl Piece for King {
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

unsafe impl Send for King {}
unsafe impl Sync for King {}