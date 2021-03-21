#[derive(Copy, Clone)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub enum Field {
    Filled(Piece, Color),
    Empty,
}

pub struct Game {
    pub board: [[Field; 8]; 8],
}


impl Game {
    pub fn get_fen(&self) -> String {
        let mut fen = "".to_string();
        let mut count: usize;

        for y in 0..self.board.len() {
            count = 0;
            for x in 0..self.board.len() {
                match &self.board[y][x] {
                    //if piece is filled add the current count to the FEN
                    Field::Filled(piece, color) => {
                        //check if letter should be capital
                        let capital = match color {
                            Color::Black => false,
                            Color::White => true,
                        };

                        if count != 0 {
                            fen += &count.to_string();
                            count = 0;
                        }
                        fen += match piece {
                            Piece::King    if !capital => "k",
                            Piece::King    if capital => "K",
                            Piece::Queen   if !capital => "q",
                            Piece::Queen   if capital => "Q",
                            Piece::Bishop  if !capital => "b",
                            Piece::Bishop  if capital => "B",
                            Piece::Knight  if !capital => "n",
                            Piece::Knight  if capital => "N",
                            Piece::Rook    if !capital => "r",
                            Piece::Rook    if capital => "R",
                            Piece::Pawn    if !capital => "p",
                            Piece::Pawn    if capital => "P",
                            _ => "",
                        };
                    },
                    //if piece is empty count one more
                    Field::Empty => {
                        count += 1;

                    },
                }
            }
            //add / at end of each row
            fen += "/";
        }
        fen
    }
}

pub const INIT_BOARD: [[Field; 8]; 8] =
    [[Field::Filled(Piece::Rook, Color::Black), Field::Filled(Piece::Knight, Color::White), Field::Empty, Field::Filled(Piece::Queen, Color::White), Field::Empty, Field::Empty, Field::Empty, Field::Empty],
     [Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty],
     [Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty],
     [Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty],
     [Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty],
     [Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty],
     [Field::Filled(Piece::Rook, Color::Black), Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Filled(Piece::Rook, Color::Black), Field::Empty, Field::Empty],
     [Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty, Field::Empty]];
