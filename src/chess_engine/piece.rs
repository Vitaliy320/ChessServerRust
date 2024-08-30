use std::fmt;

pub trait Piece: fmt::Debug + Send + Sync {

    fn get_possible_moves(&self) -> &Vec<String>;
    fn set_possible_moves(&mut self, moves: Vec<String>);
    fn calculate_possible_moves(&mut self);
    fn get_symbol(&self) -> String;
}

#[derive(Debug)]
pub struct DefaultPiece {
    possible_moves: Vec<String>,
    symbol: String,
}

impl DefaultPiece {
    pub fn new() -> DefaultPiece {
        DefaultPiece {
            possible_moves: Vec::new(),
            symbol: "d".to_string(),
        }
    }
}

impl Piece for DefaultPiece {
    fn get_possible_moves(&self) -> &Vec<String> {
        &self.possible_moves
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {}

    fn calculate_possible_moves(&mut self) {
        self.possible_moves = Vec::new()
    }

    fn get_symbol(&self) -> String {
        self.symbol.clone()
    }
}

unsafe impl Send for DefaultPiece {}
unsafe impl Sync for DefaultPiece {}