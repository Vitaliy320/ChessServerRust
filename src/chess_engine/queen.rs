use crate::chess_engine::piece::Piece;

#[derive(Debug)]
pub struct Queen {
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Queen {
    pub fn new(color: char) -> Queen {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "Q".to_string()
        } else {
            "q".to_string()
        };
        Queen {color: color,
            possible_moves: Vec::new(),
            name: String::from("Queen"),
            symbol: symbol
        }
    }
}

impl Piece for Queen {
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

unsafe impl Send for Queen {}
unsafe impl Sync for Queen {}