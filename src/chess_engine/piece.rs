pub trait Piece: Send + Sync {
    fn new(coordinates: (char, char), color: char) -> Self;
    fn get_possible_moves(&self) -> &Vec<String>;
    fn set_possible_moves(&mut self, moves: Vec<String>);
    fn calculate_possible_moves(&mut self);
    fn get_symbol(&self) -> String;
    fn get_color(&self) -> char;
    // fn as_mut(self: Box<Self>) -> Box<(dyn Piece)>;
    fn get_coordinates_string(&self) -> String;
    fn set_coordinates(&mut self, coordinates: (char, char));
    fn set_coordinates_string(&mut self, coordinates: String);
    fn get_name(&self) -> String;
    // fn as_mut(self: Box<Rook>) -> Box<Rook>;
}