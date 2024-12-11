use std::char::from_u32;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::hash::Hash;
use std::ops::Index;
use std::sync::Arc;
// use crate::chess_engine::piece::Piece;
use crate::chess_engine::piece::PieceEnum;
use crate::chess_engine::coordinates::Coordinates;
use crate::chess_engine::color::{ActiveColor};
use pleco::{Board as StockfishBoard, BitMove};

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
    w_king_square: Option<Coordinates>,
    b_king_square: Option<Coordinates>,
    w_king_in_check: bool,
    b_king_in_check: bool,
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
            // todo: update to get kings positions from fen
            w_king_square: None,
            b_king_square: None,
            w_king_in_check: false,
            b_king_in_check: false,
        };
        board.create_pieces_from_fen(fen);

        let color = board.active_color.clone();
        board.generate_possible_moves(&color, &true);
        board.generate_possible_moves(&color.next(), &true);
        // println!("board possible moves: {:?}", board.possible_moves);

        board.update_check_status(&color);
        board.update_check_status(&color.next());

        board.generate_castle_moves(&color);
        // println!("board possible moves: {:?}", board.possible_moves);
        println!("white king in check: {}, black king in check: {}", board.w_king_in_check, board.b_king_in_check);
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
        //todo: update to get w_king_square and b_king_square from pieces
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
            w_king_square: None,
            b_king_square: None,
            w_king_in_check: false,
            b_king_in_check: false,
        };

        for row in board.rows.chars() {
            for column in board.columns.chars() {
                let coordinates = Coordinates::new_from_char(&column, &row);
                if !board.pieces.contains_key(&coordinates) {
                    board.pieces.insert(coordinates, None);
                }
            }
        }

        let color = board.active_color.clone();
        board.castle_options = board.get_castle_options_by_rook_starting_squares();
        board.generate_possible_moves(&color, &true);
        board.generate_possible_moves(&color.next(), &true);

        board.update_check_status(&color);
        board.update_check_status(&color.next());

        board.generate_castle_moves(&color);

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
                    if &piece.get_symbol() == "K" && &piece.get_color() == &'w' {
                        self.w_king_square = Some(coordinates.clone());
                    }
                    if &piece.get_symbol() == "k" && &piece.get_color() == &'b' {
                        self.b_king_square = Some(coordinates.clone());
                    }

                    self.pieces.insert(coordinates, Some(piece));

                    column_counter += 1;
                }
            }
        }
        self.castle_options = self.get_castle_options_by_rook_starting_squares();
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

    pub fn update_active_color(&mut self) {
        self.active_color = self.active_color.next()
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
                            || !piece.get_possible_moves().contains(&move_to.to_string()) {
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
                        || !piece.get_possible_moves().contains(&move_to.to_string()) {
                        return false;
                    }

                    if piece.get_symbol() == "K" && piece.get_color() == 'w' {
                        self.w_king_square = Some(move_to.clone());
                        if (move_to.column - move_from.column).abs() == 2 {
                            self.castle_rook(&ActiveColor::new_from_char('w').unwrap(), move_to);
                            self.castle_options = self.castle_options.chars().filter(|c| !c.is_uppercase()).collect();
                        }
                    }

                    if piece.get_symbol() == "k" && piece.get_color() == 'b' {
                        self.b_king_square = Some(move_to.clone());
                        if (move_to.column - move_from.column).abs() == 2 {
                            self.castle_rook(&ActiveColor::new_from_char('b').unwrap(), move_to);
                            self.castle_options = self.castle_options.chars().filter(|c| !c.is_lowercase()).collect();
                        }
                    }

                    if ["R", "r"].contains(&piece.get_symbol().as_str()) {
                        self.castle_options = self.update_castle_options_after_rook_move(move_from);
                    }

                    piece.set_coordinates(&move_to);
                    self.pieces.insert(move_from.clone(), None);
                    self.pieces.insert(move_to.clone(), Some(piece));
                },
                _ => return false,
            }
        }

        if calculate_new_moves {
            self.update_check_status(&self.active_color.clone());
            self.active_color = self.active_color.next();

            let color_clone = self.active_color.clone();
            self.generate_possible_moves(&color_clone, &true);
            self.generate_possible_moves(&color_clone.next(), &false);
            self.update_check_status(&color_clone);

            self.generate_castle_moves(&color_clone);

            let s = format!("{}{}", move_from.to_string(), move_to.to_string());
            println!("make_move_str square: {}", s);
        }
        self.fen = self.board_to_fen();
        println!("white king in check: {}, black king in check: {}", self.w_king_in_check, self.b_king_in_check);
        true
    }

    pub fn castle_rook(&mut self, king_color: &ActiveColor, move_to: &Coordinates) {
        let rook_column: i8;
        let rook_new_column: i8;
        if move_to.column < (move_to.column - 7).abs() {
            rook_column = 0;
            rook_new_column = 3;
        } else {
            rook_column = 7;
            rook_new_column = 5;
        }

        match king_color {
            ActiveColor::White => {
                self.make_move_without_move_validation(
                    &Coordinates::new_from_int(&rook_column, &0),
                    &Coordinates::new_from_int(&rook_new_column, &0),
                    false,
                );
            },
            ActiveColor::Black => {
                self.make_move_without_move_validation(
                    &Coordinates::new_from_int(&rook_column, &7),
                    &Coordinates::new_from_int(&rook_new_column, &7),
                    false,
                );
            },
        }
    }

    pub fn update_castle_options_after_rook_move(
        &self,
        move_from: &Coordinates
    ) -> String {
        match (move_from.column, move_from.row) {
            (0, 0) => self.castle_options.chars().filter(|c| *c != 'Q').collect(),
            (7, 0) => self.castle_options.chars().filter(|c| *c != 'K').collect(),
            (0, 7) => self.castle_options.chars().filter(|c| *c != 'q').collect(),
            (7, 7) => self.castle_options.chars().filter(|c| *c != 'k').collect(),
            _ => self.castle_options.clone(),
        }
    }

    pub fn add_move_to_possible_moves(&mut self, move_from: &String, move_to: &String) {
        match self.possible_moves.get_mut(move_from) {
            Some((_, moves)) => {
                moves.push(move_to.clone());
            },
            None => {
                let piece = self.pieces.get(&Coordinates::new_from_string(move_from).unwrap()).unwrap();
                match piece {
                    Some(piece) => {
                        let mut moves_vector: Vec<String> = Vec::new();
                        moves_vector.push(move_to.clone());
                        self.possible_moves.insert(move_from.clone(), (piece.get_symbol(), moves_vector));
                    },
                    _ => {},
                }
            },
        }
    }

    pub fn get_castle_options_by_rook_starting_squares(&self) -> String {
        let mut coordinates: Coordinates;
        let mut castle_options = self.castle_options.clone();
        let castle_symbols = ['Q', 'K', 'q', 'k'];
        let rooks = ['R', 'R', 'r', 'r'];
        let mut piece_option;

        for (index, (column, row)) in [(0i8, 0i8), (7i8, 0i8), (0i8, 7i8), (7i8, 7i8)].iter().enumerate() {
            coordinates = Coordinates::new_from_int(&column, &row);
            piece_option = self.pieces.get(&coordinates);
            match piece_option {
                Some(piece_option) => {
                    match piece_option {
                        Some(piece) => {
                            if piece.get_symbol() != rooks[index].to_string() {
                                castle_options = castle_options.chars().filter(|c| *c != castle_symbols[index]).collect();
                            }
                        },
                        _ => castle_options = castle_options.chars().filter(|c| *c != castle_symbols[index]).collect(),
                    }
                },
                _ => castle_options = castle_options.chars().filter(|c| *c != castle_symbols[index]).collect(),

            }
        }
        castle_options
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

    pub fn make_move_without_move_validation(
        &mut self,
        move_from: &Coordinates,
        move_to: &Coordinates,
        update_color: bool,
    ) {
        match self.pieces.get_mut(&move_from) {
            Some(piece) => {
                match piece.take() {
                    Some(mut piece) => {
                        if piece.get_symbol() == "K" && piece.get_color() == 'w' {
                            self.w_king_square = Some(move_to.clone());
                        }

                        if piece.get_symbol() == "k" && piece.get_color() == 'b' {
                            self.b_king_square = Some(move_to.clone());
                        }
                        piece.set_coordinates(&move_to);
                        self.pieces.insert(move_from.clone(), None);
                        self.pieces.insert(move_to.clone(), Some(piece.clone()));
                        if update_color {
                            self.active_color = self.active_color.next();
                        }
                        self.generate_possible_moves(&self.active_color.clone(), &false);
                    },
                    _ => { },
                }
            }

            _ => { },
        };
    }

    pub fn update_check_status(&mut self, color: &ActiveColor) {
        let mut board_clone = self.clone();

        let (coordinates_to_check, mut king_in_check) = match color {
            ActiveColor::White => (&board_clone.w_king_square, &mut self.w_king_in_check),
            ActiveColor::Black => (&board_clone.b_king_square, &mut self.b_king_in_check),
        };

        for piece in self.pieces.values_mut() {
            match piece {
                Some(ref mut piece) => {
                    if piece.get_color() != color.to_char() {
                        if let Some(coordinates_to_check) = coordinates_to_check {
                            println!("piece: {}, piece possible moves: {:?}", piece.get_symbol(), piece.get_possible_moves());
                            if piece.get_possible_moves().contains(&(*coordinates_to_check).to_string()) {
                                *king_in_check = true;
                                return
                            }
                        }
                    }
                },
                _ => {},
            }
        }

        *king_in_check = false;
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
                //todo: remove next line
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

    pub fn board_to_dict(&mut self) -> HashMap<String, (String, Vec<String>)> {
        let mut dict: HashMap<String, (String, Vec<String>)> = HashMap::new();
        // todo: add calculation of possible moves for each piece in given position

        for (coordinates, piece) in self.get_pieces_dict() {
            match piece {
                Some(p) => {
                    let piece_possible_coordinates = p.get_possible_moves();
                    let s = p.get_symbol();
                    dict.insert(coordinates.to_string(), (p.get_symbol(), piece_possible_coordinates));
                },
                _ => (),
            }
        }
        dict
    }

    pub fn board_to_dict_by_active_color(&mut self) -> HashMap<String, (String, Vec<String>)> {
        let mut dict: HashMap<String, (String, Vec<String>)> = HashMap::new();
        // todo: add calculation of possible moves for each piece in given position

        for (coordinates, piece) in self.get_pieces_dict() {
            match piece {
                Some(p) => {
                    let piece_possible_coordinates = p.get_possible_moves();
                    let s = p.get_symbol();
                    if p.get_color() == self.active_color.to_char() {
                        dict.insert(coordinates.to_string(), (p.get_symbol(), piece_possible_coordinates));
                    } else {
                        dict.insert(coordinates.to_string(), (p.get_symbol(), Vec::new()));
                    }

                },
                _ => (),
            }
        }
        dict
    }

    pub fn board_dict_to_string(&self, columns: String, rows: String, board: HashMap<String, (String, Vec<String>)>) -> String {
        let board_string: String = rows.chars()
            .rev()
            .enumerate()
            .map(|(row_index, row)| {
                let mut row_string = format!("{} ", 8 - row_index);
                row_string.push_str(&columns.chars()
                    .map(|col| {
                        let coordinates = format!("{}{}", col, row);
                        match board.get(&coordinates) {
                            Some((piece, possible_moves)) => format!("{} ", piece.to_string()),
                            None => "  ".to_string(),
                        }
                    })
                    .collect::<String>()
                );
                row_string + "\n"
            })
            .collect();

        format!("{}{}", board_string, format!("  {}", columns.chars()
            .map(|column| {
                format!("{} ", column.to_uppercase())
            }).collect::<String>()
        ))
    }

    fn generate_possible_moves(&mut self, color: &ActiveColor, calculate_check_moves: &bool) {
        // let stockfish_board = StockfishBoard::from_fen(self.get_fen().as_str()).unwrap();
        // let moves = stockfish_board.generate_moves();
        // let mut move_from: String;
        // let mut move_to: String;
        //
        // for piece_move in moves {
        //     let piece_move = &piece_move.to_string();
        //     (move_from, move_to) = (piece_move[0..2].to_string(), piece_move[2..4].to_string());
        //     println!("move_from: {}, move_to: {}", move_from, move_to);
        //     // self.add_move_to_possible_moves(&move_from, &move_to);
        //
        //     // let mut piece = self.pieces.get_mut(&Coordinates::new_from_string(move_from).unwrap()).unwrap();
        //     // match piece {
        //     //     Some(piece) => {
        //     //         piece.get_possible_moves().push(move_to.clone());
        //     //         self.pieces
        //     //     },
        //     // }
        //     // println!("move: {}", pos_move.to_string());
        // }

        // println!("moves: {:?}", moves);
        let mut validate_moves: Vec<String> = Vec::new();
        let board_clone = &self.clone();
        for piece in self.pieces.values_mut() {
            if let Some(piece) = piece {
                if piece.get_color() == color.to_char() {
                    let possible_moves = piece.generate_piece_moves(board_clone, color, calculate_check_moves);
                    // println!("board:\n{}\nactive color: {}\npiece: {}\npossible moves: {:?}",
                    //          board_clone.clone().board_to_string(), self.active_color.to_char(), piece.get_symbol(), possible_moves);
                    // println!("possible_moves: {:?}", possible_moves);
                    for m in &possible_moves {
                        validate_moves.push(format!("{}{}", piece.get_coordinates_string(), m));
                    }
                    self.possible_moves.insert(
                        piece.get_coordinates_string(),
                        (piece.get_symbol(), possible_moves)
                    );
                }
            }
        }
        println!("board.possible_moves: {:?}", self.possible_moves)
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
                if p.get_color() != *color {
                    return true;
                }
                false
            },
            _ => false,
        }
    }

    pub fn square_contains_piece_of_same_color(&self, coordinates: &Coordinates, color: &char) -> bool {
        match self.pieces.get(&coordinates) {
            Some(res) => {
                match res {
                    Some(p) => {
                        if p.get_color() == *color {
                            return true
                        }
                        false
                    },
                    _ => false,
                }
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

    pub fn king_in_check_after_move(
        &self,
        from_coordinates: &Coordinates,
        to_coordinates: &Coordinates,
        king_color: &ActiveColor,
        // calculate_check_moves: &bool,
    ) -> bool {
        // match calculate_check_moves {
        //     true => {
        //     let king_color = self.get_active_color();
            // println!("board before move: {:?}", self.get_pieces_vec());
            let mut board_after_move = self.clone();
            board_after_move.make_move_without_move_validation(from_coordinates, to_coordinates, true);
            // println!("board before move:\n{}", &self.clone().board_to_string());
            // let mut board_after_move = self.clone();
            // match board_after_move.pieces.get_mut(&from_coordinates) {
            //     Some(piece) => {
            //         match piece.take() {
            //             Some(mut piece) => {
            //                 piece.set_coordinates(&to_coordinates);
            //                 board_after_move.pieces.insert(to_coordinates.clone(), Some(piece.clone()));
            //                 board_after_move.pieces.insert(from_coordinates.clone(), None);
            //             },
            //             _ => return false,
            //         }
            //     }
            //
            //     _ => return false,
            // };
            // println!("board before move:\n{},\nboard after move:\n{}", &self.clone().board_to_string(), board_after_move.board_to_string());
            // println!("board after move: {:?}", board_after_move.get_pieces_vec());
            // board_after_move.update_active_color();
            board_after_move.king_is_in_check(&king_color)
            // },
            // false => false,
        // }
    }

    pub fn king_is_in_check(&self, color: &ActiveColor) -> bool {
        let king_square = match color {
            ActiveColor::White => &self.w_king_square.to_owned().unwrap(),
            ActiveColor::Black => &self.b_king_square.to_owned().unwrap(),
        };

        self.square_is_attacked(king_square, color)
    }

    pub fn square_is_attacked(&self, coordinates: &Coordinates, color: &ActiveColor) -> bool {
        let color_char = color.to_char();
        for piece in self.pieces.values() {
            match piece {
                Some(piece) => {
                    if piece.get_color() != color_char {
                        let mut new_piece = piece.clone();
                        // println!("Check validation: {}, {}", new_piece.get_symbol(), new_piece.get_color());
                        new_piece.generate_piece_moves(&self, color, &false);
                        // println!("Possible moves: {:?}", new_piece.get_possible_moves());
                        if new_piece.get_possible_moves().contains(&coordinates.to_string()) {
                            return true;
                        }
                    }
                },
                _ => {},
            }
        }
        false
    }

    pub fn square_is_attacked_new_board(&self, coordinates: &Coordinates, color: &ActiveColor) -> bool {
        let mut new_board = self.clone();
        new_board.active_color = new_board.active_color.next();
        let color_char = color.to_char();
        for piece in new_board.pieces.values() {
            match piece {
                Some(piece) => {
                    if piece.get_color() != color_char {
                        let mut new_piece = piece.clone();
                        // println!("Check validation: {}, {}", new_piece.get_symbol(), new_piece.get_color());
                        new_piece.generate_piece_moves(&new_board, color, &false);
                        // println!("Possible moves: {:?}", new_piece.get_possible_moves());
                        if new_piece.get_possible_moves().contains(&coordinates.to_string()) {
                            return true;
                        }
                    }
                },
                _ => {},
            }
        }
        false
    }

    pub fn square_is_visible(&self, coordinates: &Coordinates, color: &ActiveColor) -> bool {
        let color_char = color.to_char();
        let mut piece_enum: PieceEnum;
        for piece in self.pieces.values() {
            match piece {
                Some(piece) => {
                    piece_enum = piece.clone();
                    piece_enum.generate_piece_moves(&self, &color.next(), &false);
                    if piece_enum.get_color() != color_char && piece_enum.get_possible_moves().contains(&coordinates.to_string()) {
                        return true;
                    }
                },
                _ => {},
            }
        }
        false
    }

    pub fn kings_adjacent(&self, square_to_check: &Coordinates) -> bool {
        let another_king_square = match self.active_color {
            ActiveColor::White => &self.b_king_square,
            ActiveColor::Black => &self.w_king_square,
        };

        let distance_between_squares = (
            (square_to_check.column - another_king_square.to_owned().unwrap().column).abs(),
            (square_to_check.row - another_king_square.to_owned().unwrap().row).abs(),
        );

        if distance_between_squares.0 <= 1 && distance_between_squares.1 <= 1 {
            return true;
        }
        false
    }

    pub fn distance_between_coordinates(&self, square1: &Coordinates, square2: &Coordinates) -> (u32, u32) {
        (
            (square1.column - square2.column) as u32,
            (square1.row - square2.row) as u32,
        )
    }

    pub fn generate_castle_moves(&mut self, active_color: &ActiveColor) {
        match active_color {
            ActiveColor::White => {
                if self.w_king_in_check {
                    return;
                }
            },
            ActiveColor::Black => {
                if self.b_king_in_check {
                    return;
                }
            }
        }

        let (king_square, castle_fen) = match active_color {
            ActiveColor::White => (&self.w_king_square.to_owned().unwrap(), ['K', 'Q']),
            ActiveColor::Black => (&self.b_king_square.to_owned().unwrap(), ['k', 'q'])
        };

        for (index, castle_move) in [1i8, -1i8].iter().enumerate() {
            let square1 = &Coordinates::new_from_int(
                &(king_square.column + castle_move),
                &king_square.row,
            );
            let square2 = &Coordinates::new_from_int(
                &(king_square.column + castle_move * 2),
                &king_square.row,
            );

            if self.castle_options.contains(castle_fen[index]) &&
                self.square_is_free(square1) &&
                !self.square_is_visible(square1, active_color) &&
                self.square_is_free(square2) &&
                !self.square_is_visible(square2, active_color) {

                let king_piece = self.pieces.get_mut(king_square).unwrap();
                match king_piece {
                    Some(king_piece) => {
                        let mut moves = king_piece.get_possible_moves();
                        moves.push(Coordinates::new_from_int(
                            &(king_square.column + castle_move * 2),
                            &king_square.row,
                        ).to_string());
                        king_piece.set_possible_moves(moves);
                        // castle_squares[index] = Some(Coordinates::new_from_int(
                        //     &(king_square.column + castle_move * 2),
                        //     &king_square.row,
                        // ));
                    },
                    _ => {},
                }
            }
        }
    }
}

