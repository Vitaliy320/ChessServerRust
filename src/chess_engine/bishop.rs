use crate::chess_engine::piece::Piece;

#[derive(Debug)]
pub struct Bishop {
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Bishop {
    pub fn new(color: char) -> Bishop {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "B".to_string()
        } else {
            "b".to_string()
        };
        Bishop {color: color,
            possible_moves: Vec::new(),
            name: String::from("Bishop"),
            symbol: symbol
        }
    }
}

impl Piece for Bishop {
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

unsafe impl Send for Bishop {}
unsafe impl Sync for Bishop {}