#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Coordinates {
    pub column: i8,
    pub row: i8,
    pub column_char: char,
    pub row_char: char,
}

impl Coordinates {
    pub fn new_from_int(column: &i8, row: &i8) -> Coordinates {
        Coordinates {
            column: *column,
            row: *row,
            column_char: char::from_digit(*column as u32, 10).unwrap(),
            row_char: char::from_digit(*row as u32, 10).unwrap(),
        }
    }
    pub fn new_from_char(column: &char, row: &char) -> Coordinates {
        Coordinates::new_from_int(&(*column as i8 - 'a' as i8), &(*row as i8 - '1' as i8))
    }

    pub fn new_from_string(coordinates: &String) -> Option<Coordinates> {
        match coordinates.len() {
            2 => Some(Coordinates::new_from_int(
                &(coordinates.chars().nth(0)? as i8 - 'a' as i8),
                &(coordinates.chars().nth(1)? as i8 - '1' as i8),
            )),
            _ => None,
        }
    }

    pub fn get_coordinates_int(&self) -> (i8, i8) {
        (self.column, self.row)
    }

    pub fn get_coordinates_char(&self) -> (char, char) {
        (char::from_digit(self.column as u32, 10).unwrap(),
         char::from_digit(self.row as u32, 10).unwrap())
    }

    pub fn get_coordinates_string(&self) -> String {
        format!("{}{}", self.column.to_string(), self.row.to_string())
    }
}