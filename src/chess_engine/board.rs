use std::char::from_u32;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::hash::Hash;
use std::sync::Arc;
use crate::chess_engine::piece::Piece;
use crate::chess_engine::piece::PieceEnum;
use crate::chess_engine::coordinates::Coordinates;
use crate::chess_engine::color::ActiveColor;

#[derive(Debug, Clone)]
pub struct Board {
    id: Option<i32>,
    fen: String,
    pieces: HashMap<Coordinates, Option<PieceEnum>>,
    possible_moves: HashMap<String, (String, Vec<String>)>,
    active_color: ActiveColor,
    castle_options: String,
    en_passant_square: String,
    half_move_clock: i32,
    full_move_number: i32,
    number_of_columns: u32,
    number_of_rows: u32,
    columns: String,
    rows: String,
    columns_set: HashSet<char>,
    rows_set: HashSet<char>,
    bottom_left_square: Coordinates,
    top_right_square: Coordinates,
    w_king_position: Coordinates,
    b_king_position: Coordinates,
}

impl Board {
    pub fn new_from_fen(
        columns: String,
        n_of_columns: u32,
        rows: String,
        n_of_rows: u32,
        fen: String,
    ) -> Self {
        let mut board = Board {
            id: None,
            fen: fen.clone(),
            pieces: HashMap::new(),
            possible_moves: HashMap::new(),
            number_of_columns: n_of_columns,
            number_of_rows: n_of_rows,
            columns: columns.clone(),
            rows: rows.clone(),
            active_color: ActiveColor::White,
            castle_options: "".to_string(),
            en_passant_square: "".to_string(),
            half_move_clock: 0,
            full_move_number: 0,
            columns_set: HashSet::from_iter(columns.chars()),
            rows_set: HashSet::from_iter(rows.chars()),
            bottom_left_square: Coordinates::new_from_char(
                &(columns.chars().nth(0).unwrap()),
                &(rows.chars().nth(0).unwrap())
            ),
            top_right_square: Coordinates::new_from_char(
                &(columns.chars().rev().nth(0).unwrap()),
                &(rows.chars().rev().nth(0).unwrap())
            ),
            w_king_position: Coordinates::new_from_char(&'e', &'1'),
            b_king_position: Coordinates::new_from_char(&'e', &'8'),
        };
        board.create_pieces_from_fen(fen);
        // board.pieces = board.create_pieces();
        board.calculate_possible_moves();
        board
    }

    pub fn new_from_db(
        id: i32,
        fen: String,
        pieces: HashMap<Coordinates, Option<PieceEnum>>,
        active_color: char,
        castle_options: String,
        en_passant_square: String,
        half_move_clock: i32,
        full_move_number: i32,
        number_of_columns: i32,
        number_of_rows: i32,
        columns: String,
        rows: String,
    ) -> Board {
        let active_color_enum = match active_color {
            'b' => ActiveColor::Black,
            _ => ActiveColor::White,
        };
        let mut board = Board {
            id: Some(id),
            fen,
            pieces,
            possible_moves: HashMap::new(),
            active_color: active_color_enum,
            castle_options,
            en_passant_square,
            half_move_clock,
            full_move_number,
            number_of_columns: number_of_columns as u32,
            number_of_rows: number_of_rows as u32,
            columns: columns.clone(),
            rows: rows.clone(),
            columns_set: HashSet::from_iter(columns.chars()),
            rows_set: HashSet::from_iter(rows.chars()),
            bottom_left_square: Coordinates::new_from_char(
                &(columns.chars().nth(0).unwrap()),
                &(rows.chars().nth(0).unwrap())
            ),
            top_right_square: Coordinates::new_from_char(
                &(columns.chars().rev().nth(0).unwrap()),
                &(rows.chars().rev().nth(0).unwrap())
            ),
            w_king_position: Coordinates::new_from_char(&'e', &'1'),
            b_king_position: Coordinates::new_from_char(&'e', &'8'),
        };

        for row in board.rows.chars() {
            for column in board.columns.chars() {
                let coordinates = Coordinates::new_from_char(&column, &row);
                if !board.pieces.contains_key(&coordinates) {
                    board.pieces.insert(coordinates, None);
                }
            }
        }
        board.calculate_possible_moves();
        board
    }

    pub fn board_to_fen(&self) -> String {
        let mut board_fen = String::new();
        let mut coordinates: Coordinates;
        for (row_index, row_coordinate) in self.rows.chars().rev().enumerate() {
            let mut empty_cells_counter = 0;
            for column_coordinate in self.columns.chars() {
                coordinates = Coordinates::new_from_char(&column_coordinate, &row_coordinate);
                let piece = self.pieces.get(&coordinates).unwrap();
                match piece {
                    Some(piece) => {
                        if empty_cells_counter > 0 {
                            board_fen.push(std::char::from_digit(empty_cells_counter, 10).unwrap());
                        }
                        empty_cells_counter = 0;
                        board_fen.push_str(piece.get_symbol().as_str());
                    },
                    None => empty_cells_counter += 1,
                }
            }

            if empty_cells_counter > 0 {
                board_fen.push(std::char::from_digit(empty_cells_counter, 10).unwrap());
            }

            if row_index < self.rows.len() - 1 {
                board_fen.push('/');
            }
        }

        let fen = format!("{} {} {} {} {} {}", board_fen, self.active_color.to_char(), self.castle_options, self.en_passant_square, self.half_move_clock, self.full_move_number);
        fen
    }

    pub fn create_pieces_from_fen(&mut self, fen: String) {
        let split_fen: Vec<String>        = fen.split(' ').map(|s| String::from(s)).collect();
        let board_fen             = &split_fen[0];
        let color_fen             = &split_fen[1];
        let castle_fen            = &split_fen[2];
        let en_passant_fen        = &split_fen[3];
        let half_move_fen         = &split_fen[4];
        let full_move_number_fen  = &split_fen[5];

        self.active_color = match color_fen.chars().nth(0).unwrap() {
            'b' => ActiveColor::Black,
            _ => ActiveColor::White,
        };
        self.castle_options = castle_fen.clone();
        self.en_passant_square = en_passant_fen.clone();
        self.half_move_clock = half_move_fen.parse().unwrap();
        self.full_move_number = full_move_number_fen.parse().unwrap();

        let mut board_rows: Vec<String> = board_fen.split('/')
            .map(|s| String::from(s)).collect();
        board_rows.reverse();

        let mut row: &String;
        let mut piece: PieceEnum;
        let mut column_counter: i32;
        let mut column_coordinate: char;
        let mut row_coordinate: char;


        for j in 0..board_rows.len() {
            row = &board_rows[j];
            row_coordinate = self.rows.chars().nth(j).unwrap();
            column_counter = 0;

            for cell_value in row.chars() {
                // if a row consists of piece symbols and numbers,
                // for example, 4p2p, four empty squares, a pawn, two empty squares,
                // and a pawn will be created.
                if cell_value.is_digit(10) {
                    let n_iter = column_counter + cell_value.to_digit(10).unwrap_or(0) as i32;
                    for i in column_counter..n_iter {
                        column_coordinate = self.columns.chars().nth(i as usize).unwrap();
                        let coordinates = Coordinates::new_from_char(&column_coordinate, &row_coordinate);
                        self.pieces.insert(coordinates, None);
                    }
                    column_counter += cell_value as i32 - '0' as i32;
                } else {
                    column_coordinate = self.columns.chars()
                        .nth(column_counter as usize)
                        .unwrap();
                    let coordinates = Coordinates::new_from_char(&column_coordinate, &row_coordinate);
                    piece = PieceEnum::new(coordinates.clone(), cell_value);
                    self.pieces.insert(coordinates, Some(piece));
                    column_counter += 1;
                }
            }
        }
    }


    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn get_fen(&self) -> String {
        self.fen.clone()
    }

    pub fn get_pieces_dict(&self) -> HashMap<Coordinates, Option<PieceEnum>> {
        self.pieces.clone()
    }

    pub fn get_mut_pieces_dict(&mut self) -> &mut HashMap<Coordinates, Option<PieceEnum>> {
        &mut self.pieces
    }

    pub fn get_pieces_vec(&self) -> Vec<PieceEnum> {
        let pieces: Vec<PieceEnum> = self.pieces.clone()
            .into_values()
            .filter_map(|piece|
                piece.map(|boxed_piece| boxed_piece))
            .collect();

        pieces
    }

    pub fn get_mut_pieces_vec(&mut self) -> Vec<&mut PieceEnum> {
        let pieces: Vec<&mut PieceEnum> = self.pieces
            .values_mut()
            .filter_map(|piece| piece.as_mut())
            .collect();

        pieces
    }

    pub fn get_active_color(&self) -> ActiveColor {
        self.active_color.clone()
    }

    pub fn get_active_color_string(&self) -> String {
        match self.active_color {
            ActiveColor::White => "w".to_string(),
            ActiveColor::Black => "b".to_string(),
        }
    }

    pub fn get_castle_options(&self) -> String {
        self.castle_options.clone()
    }

    pub fn get_en_passant_square(&self) -> String {
        self.en_passant_square.clone()
    }

    pub fn get_half_move_clock(&self) -> i32 {
        self.half_move_clock.clone()
    }

    pub fn get_full_move_number(&self) -> i32 {
        self.full_move_number.clone()
    }

    pub fn get_number_of_columns(&self) -> u32 {
        self.number_of_columns.clone()
    }

    pub fn get_number_of_rows(&self) -> u32 {
        self.number_of_rows.clone()
    }

    pub fn get_columns(&self) -> String {
        self.columns.clone()
    }

    pub fn get_rows(&self) -> String {
        self.rows.clone()
    }

    pub fn get_columns_set(&self) -> &HashSet<char> {
        &self.columns_set
    }

    pub fn get_rows_set(&self) -> &HashSet<char> {
        &self.rows_set
    }

    pub fn make_move(
        &mut self,
        move_from: &Coordinates,
        move_to: &Coordinates,
        calculate_new_moves: bool,
    ) -> bool {
        // todo: update board in database
        {
            if let Some(piece_option) = self.pieces.get(&move_from) {
                match piece_option {
                    Some(piece) => {
                        let a = piece.get_possible_moves();
                        if piece.get_color() != self.active_color.to_char()
                            || !piece.get_possible_moves().contains(&move_to.get_coordinates_string()) {
                            return false;
                        }
                    },
                    _ => return false,
                }
            }
        }
        if let Some(piece_option) = self.pieces.get_mut(&move_from) {
            match piece_option.take() {
                Some(mut piece) => {
                    if piece.get_color() != self.active_color.to_char()
                        || !piece.get_possible_moves().contains(&move_to.get_coordinates_string()) {
                        return false;
                    }
                    if piece.get_symbol() == "K" && piece.get_color() == 'w' {
                        self.w_king_position = move_to.clone();
                    }

                    if piece.get_symbol() == "k" && piece.get_color() == 'b' {
                        self.w_king_position = move_to.clone();
                    }

                    piece.set_coordinates(&move_to);
                    self.pieces.insert(move_to.clone(), Some(piece));
                    self.pieces.insert(move_from.clone(), None);
                },
                _ => return false,
            }
        }

        if calculate_new_moves {
            self.active_color = self.active_color.next();
            self.calculate_possible_moves();
            let s = format!("{}{}", move_from.get_coordinates_string(), move_to.get_coordinates_string());
            println!("make_move_str square: {}", s);
        }
        self.fen = self.board_to_fen();
        true
    }

    pub fn make_move_string(&mut self, move_from: String, move_to: String) -> bool {

        if (move_from.len(), move_to.len()) != (2, 2)
            || !self.columns_set.contains(&move_from.chars().nth(0).unwrap())
            || !self.rows_set.contains(&move_from.chars().nth(1).unwrap())
            || !self.columns_set.contains(&move_to.chars().nth(0).unwrap())
            || !self.rows_set.contains(&move_to.chars().nth(1).unwrap()) {
            return false;
        }

        let move_from = Coordinates::new_from_string(&move_from).unwrap();
        let move_to = Coordinates::new_from_string(&move_to).unwrap();

        self.make_move(&move_from, &move_to, true)
    }

    pub fn make_move_chars(&mut self, move_from: (char, char), move_to: (char, char)) -> bool {
        self.make_move(
            &Coordinates::new_from_char(&move_from.0, &move_from.1),
            &Coordinates::new_from_char(&move_to.0, &move_to.1),
            true
        )
    }

    pub fn board_to_string(&mut self) -> String {
        //todo: refactor to get rid of self.squares_vec or remove

        let mut rows_vector: Vec<String> = Vec::new();

        let mut board_string: String = String::new();

        rows_vector.push("                  ".to_string());
        rows_vector.push("                  ".to_string());

        for (row_index, row) in self.rows.chars().rev().enumerate() {
            board_string = format!("{}\n{} ", board_string, (self.number_of_rows - row_index as u32).to_string());
            for column in self.columns.chars() {
                let coordinates = Coordinates::new_from_char(&column, &row);
                let a = self.pieces.get(&coordinates);
                match self.pieces.get(&coordinates).unwrap() {
                    Some(piece) => board_string.push_str(format!("{} ", piece.get_symbol()).as_str()),
                    None => board_string += "  ",
                }
            }
        }

        board_string.push_str("\n  A B C D E F G H ");

        board_string
    }

    fn calculate_possible_moves(&mut self) {
        let board_clone = &self.clone();
        for piece in self.pieces.values_mut() {
            if let Some(piece) = piece {
                self.possible_moves.insert(
                    piece.get_coordinates_string(),
                    (piece.get_symbol(), piece.calculate_possible_moves(board_clone))
                );
            }
        }
    }

    pub fn square_is_valid(&self, coordinates: &Coordinates) -> bool {
        if coordinates.column < self.bottom_left_square.column
            || coordinates.row < self.bottom_left_square.row
            || coordinates.column > self.top_right_square.column
            || coordinates.row > self.top_right_square.row {
            return false;
        }
        true
    }

    pub fn square_is_capturable(&self, coordinates: &Coordinates, color: &char) -> bool {
        match self.pieces.get(&coordinates).unwrap() {
            Some(p) => {
                if !["k", "K"].contains(&p.get_symbol().as_str())
                    && p.get_color() != *color {
                    false;
                }
                true
            },
            _ => false,
        }
    }

    pub fn square_is_free(&self, coordinates: &Coordinates) -> bool {
        match self.pieces.get(&coordinates).unwrap() {
            None => true,
            _ => false,
        }
    }

    pub fn king_in_check_after_move(&self, from_coordinates: &Coordinates, to_coordinates: &Coordinates) -> bool {
        let king_color = self.get_active_color();
        let mut board_after_move = self.clone();
        let _ = board_after_move.make_move(&from_coordinates, &to_coordinates, false);
        board_after_move.king_is_in_check(king_color)
    }

    pub fn king_is_in_check(&self, color: ActiveColor) -> bool {
        let king_square = match color {
            ActiveColor::White => self.w_king_position.clone(),
            ActiveColor::Black => self.b_king_position.clone(),
        };

        self.square_is_attacked(king_square, color)
    }

    pub fn square_is_attacked(&self, coordinates: Coordinates, color: ActiveColor) -> bool {
        let color = color.to_char();
        for piece in self.pieces.values() {
            match piece {
                Some(piece) => {
                    if piece.get_color() != color {
                        if piece.get_possible_moves().contains(&coordinates.get_coordinates_string()) {
                            return true;
                        }
                    }
                },
                _ => {},
            }
        }
        false
    }
}

