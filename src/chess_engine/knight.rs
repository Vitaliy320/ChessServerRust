use crate::chess_engine::piece::Piece;

#[derive(Debug)]
pub struct Knight {
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Knight {
    pub fn new(color: char) -> Knight {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "N".to_string()
        } else {
            "n".to_string()
        };
        Knight {color: color,
            possible_moves: Vec::new(),
            name: String::from("Knight"),
            symbol: symbol
        }
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
}

unsafe impl Send for Knight {}
unsafe impl Sync for Knight {}