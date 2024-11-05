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
            column_char: ('a' as i8 + *column) as u8 as char,
            row_char: char::from_digit((*row + 1) as u32, 10).unwrap(),
        }
    }

    pub fn new_from_int_limited(column: &mut i8, row: &mut i8, n_columns: i32, n_rows: i32) -> Coordinates {
        if (*column as i32) < 0 {
            *column = 0;
        }

        if (*column as i32) >= n_columns {
            *column = n_columns as i8 - 1;
        }

        if (*row as i32) < 0 {
            *row = 0;
        }

        if (*row as i32) >= n_rows {
            *row = n_rows as i8 - 1;
        }

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
        format!("{}{}", self.column_char.to_string(), self.row_char.to_string())
    }
}