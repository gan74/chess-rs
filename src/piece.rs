
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColoredPiece {
    pub piece: Piece,
    pub color: Color
}

impl Piece {
    pub fn char_for_piece(&self) -> char {
        match self {
            Piece::Empty => '.',
            Piece::Pawn => 'p',
            Piece::Rook => 'r',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Queen => 'q',
            Piece::King => 'k'
        }
    }
     
    pub fn colored(&self, col: Color) -> ColoredPiece {
        ColoredPiece {
            piece: *self,
            color: col
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Piece::Empty
    }
}

impl Color {
    pub fn inverse(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    }
}

impl ColoredPiece {
    pub fn char_for_piece(&self) -> char {
        let c = self.piece.char_for_piece();
        match self.color {
            Color::Black => c,
            Color::White => c.to_uppercase().next().unwrap_or(Piece::Empty.char_for_piece())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.piece.is_empty()
    }
}


impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.char_for_piece())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White")
        }
    }
}

impl fmt::Display for ColoredPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.char_for_piece())
    }
}