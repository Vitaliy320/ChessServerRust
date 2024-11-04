use crate::chess_engine::{
    pawn::Pawn,
    knight::Knight,
    bishop::Bishop,
    rook::Rook,
    queen::Queen,
    king::King,
};
use crate::chess_engine::coordinates::Coordinates;

#[derive(Debug, Clone)]
pub enum PieceEnum {
    Pawn(Box<Pawn>),
    Knight(Box<Knight>),
    Bishop(Box<Bishop>),
    Rook(Box<Rook>),
    Queen(Box<Queen>),
    King(Box<King>),
}

enum FunctionVariant<F, Fp, P> {
    WithoutParams(F),
    WithParams(Fp, P),
}


pub trait Piece {
    fn get_possible_moves(&self) -> Vec<String>;
    fn set_possible_moves(&mut self, moves: Vec<String>);
    fn calculate_possible_moves(&mut self, active_color: char, rows: &String, columns: &String);
    fn get_symbol(&self) -> String;
    fn get_color(&self) -> char;
    fn get_coordinates(&self) -> Coordinates;
    fn get_coordinates_string(&self) -> String;
    fn set_coordinates(&mut self, coordinates: Coordinates);
    fn set_coordinates_string(&mut self, coordinates: String);
    fn get_name(&self) -> String;
    // fn as_mut(self: Box<Rook>) -> Box<Rook>;
}

pub fn dispatch_variant<F, Fp, P>(piece: &PieceEnum, func: FunctionVariant<F, Fp, P>)
where
    F: FnMut(&dyn Piece),
    Fp: FnMut(&dyn Piece, P),
{
    match func {
        FunctionVariant::WithoutParams(mut func) => {
            match piece {
                PieceEnum::Pawn(piece) => func(piece.as_ref()),
                PieceEnum::Knight(piece) => func(piece.as_ref()),
                PieceEnum::Bishop(piece) => func(piece.as_ref()),
                PieceEnum::Rook(piece) => func(piece.as_ref()),
                PieceEnum::Queen(piece) => func(piece.as_ref()),
                PieceEnum::King(piece) => func(piece.as_ref()),
            }
        },
        FunctionVariant::WithParams(mut func, params) => {
            match piece {
                PieceEnum::Pawn(piece) => func(piece.as_ref(), params),
                PieceEnum::Knight(piece) => func(piece.as_ref(), params),
                PieceEnum::Bishop(piece) => func(piece.as_ref(), params),
                PieceEnum::Rook(piece) => func(piece.as_ref(), params),
                PieceEnum::Queen(piece) => func(piece.as_ref(), params),
                PieceEnum::King(piece) => func(piece.as_ref(), params),
            }
        },
    }
}

pub fn dispatch_variant_mut<F, Fp, P>(piece: &mut PieceEnum, func: FunctionVariant<F, Fp, P>)
where
    F: FnMut(&mut dyn Piece),
    Fp: FnMut(&mut dyn Piece, P),
{
    match func {
        FunctionVariant::WithoutParams(mut func) => {
            match piece {
                PieceEnum::Pawn(piece) => func(piece.as_mut()),
                PieceEnum::Knight(piece) => func(piece.as_mut()),
                PieceEnum::Bishop(piece) => func(piece.as_mut()),
                PieceEnum::Rook(piece) => func(piece.as_mut()),
                PieceEnum::Queen(piece) => func(piece.as_mut()),
                PieceEnum::King(piece) => func(piece.as_mut()),
            }
        },
        FunctionVariant::WithParams(mut func, params) => {
            match piece {
                PieceEnum::Pawn(piece) => func(piece.as_mut(), params),
                PieceEnum::Knight(piece) => func(piece.as_mut(), params),
                PieceEnum::Bishop(piece) => func(piece.as_mut(), params),
                PieceEnum::Rook(piece) => func(piece.as_mut(), params),
                PieceEnum::Queen(piece) => func(piece.as_mut(), params),
                PieceEnum::King(piece) => func(piece.as_mut(), params),
            }
        },
    }
}

impl PieceEnum {
    pub fn new(
        coordinates: Coordinates,
        symbol: char,
    ) -> PieceEnum {
        
        match symbol {
            'p' => PieceEnum::Pawn(Box::new(Pawn::new('b', coordinates))),
            'n' => PieceEnum::Knight(Box::new(Knight::new('b', coordinates))),
            'b' => PieceEnum::Bishop(Box::new(Bishop::new('b', coordinates))),
            'r' => PieceEnum::Rook(Box::new(Rook::new('b', coordinates))),
            'q' => PieceEnum::Queen(Box::new(Queen::new('b', coordinates))),
            'k' => PieceEnum::King(Box::new(King::new('b', coordinates))),

            'P' => PieceEnum::Pawn(Box::new(Pawn::new('w', coordinates))),
            'N' => PieceEnum::Knight(Box::new(Knight::new('w', coordinates))),
            'B' => PieceEnum::Bishop(Box::new(Bishop::new('w', coordinates))),
            'R' => PieceEnum::Rook(Box::new(Rook::new('w', coordinates))),
            'Q' => PieceEnum::Queen(Box::new(Queen::new('w', coordinates))),
            'K' => PieceEnum::King(Box::new(King::new('w', coordinates))),

            _ => PieceEnum::Pawn(Box::new(Pawn::new('w', coordinates))),
        }
    }

    pub fn get_possible_moves(&self) -> Vec<String> {
        let mut moves: Option<Vec<String>> = None;//Vec::new();
        dispatch_variant(self,
                         FunctionVariant::<_, fn(&dyn Piece, ()), (), >::WithoutParams(
                             |piece: &dyn Piece|
                            moves = Some(piece.get_possible_moves())
        ));

        moves.unwrap_or_else(|| Vec::new())
    }

    pub fn set_possible_moves(&mut self, moves: Vec<String>) {
        dispatch_variant_mut(self,
        FunctionVariant::<fn(&mut dyn Piece), _, Vec<String>>::WithParams(|piece: &mut dyn Piece, params|
            piece.set_possible_moves(params), moves),)
    }

    pub fn calculate_possible_moves(&mut self, active_color: char, rows: &String, columns: &String) {
        dispatch_variant_mut(
            self,
            FunctionVariant::<fn(&mut dyn Piece), _, (char, &String, &String)>::WithParams(
                |piece: &mut dyn Piece, (color, rows_, columns_)|
                    piece.calculate_possible_moves(color, rows_, columns_), (active_color, rows, columns)
            ),
        );
    }
    pub fn get_symbol(&self) -> String {
        let mut symbol: Option<String> = None;
        dispatch_variant(self,
        FunctionVariant::<_, fn(&dyn Piece, ()), ()>::WithoutParams(
            |piece: &dyn Piece| symbol = Some(piece.get_symbol())
        ));
        symbol.unwrap_or_else(|| String::from(""))
    }
    pub fn get_color(&mut self) -> char {
        let mut color: Option<char> = None;
        dispatch_variant(self,
                         FunctionVariant::<_, fn(&dyn Piece, ()), ()>::WithoutParams(
                             |piece: &dyn Piece| color = Some(piece.get_color())
                         ));
        color.unwrap_or_else(|| ' ')
    }

    pub fn get_coordinates(&self) -> Coordinates {
        let mut coordinates: Option<Coordinates> = None;
        dispatch_variant(
            self,
            FunctionVariant::<_, fn(&dyn Piece, ()), ()>::WithoutParams(
                |piece: &dyn Piece| coordinates = Some(piece.get_coordinates())
            )
        );
        coordinates.unwrap()
    }

    pub fn get_coordinates_string(&self) -> String {
        let mut coordinates: Option<String> = None;
        dispatch_variant(self,
                         FunctionVariant::<_, fn(&dyn Piece, ()), ()>::WithoutParams(
                             |piece: &dyn Piece| coordinates = Some(piece.get_coordinates_string())
                         ));
        coordinates.unwrap_or_else(|| String::from(""))
    }

    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        dispatch_variant_mut(self,
        FunctionVariant::<fn(&mut dyn Piece), _, Coordinates>::WithParams(
            |piece: &mut dyn Piece, params| piece.set_coordinates(params), coordinates
        ));
    }

    pub fn set_coordinates_string(&mut self, coordinates: String) {
        dispatch_variant_mut(self,
        FunctionVariant::<fn(&mut dyn Piece), _, String>::WithParams(
            |piece: &mut dyn Piece, params| piece.set_coordinates_string(params), coordinates
        ));

    }

    pub fn get_name(&mut self) -> String {
        let mut name: Option<String> = None;
        dispatch_variant(self,
                         FunctionVariant::<_, fn(&dyn Piece, ()), ()>::WithoutParams(
                             |piece: &dyn Piece| name = Some(piece.get_name())
                         ));
        name.unwrap_or_else(|| String::from(""))
    }
}