pub enum Color {
    White,
    Black,
    Random,
}

#[derive(Debug, Clone)]
pub enum ActiveColor {
    White,
    Black,
}

impl ActiveColor {
    pub fn to_char(&self) -> char {
        match self {
            ActiveColor::White => 'w',
            ActiveColor::Black => 'b',
        }
    }

    pub fn next(&self) -> ActiveColor {
        match self {
            ActiveColor::White => ActiveColor::Black,
            ActiveColor::Black => ActiveColor::White,
        }
    }

    pub fn equals(&self, color: ActiveColor) -> bool {
        match (self, color) {
            (ActiveColor::White, ActiveColor::White) | (ActiveColor::Black, ActiveColor::Black) => true,
            _ => false,
        }
    }
}