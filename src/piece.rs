use std::fmt;



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}


impl PieceKind {
    pub fn to_char(&self) -> char {
        match self {
            &PieceKind::None => '.',
            &PieceKind::Pawn => 'p',
            &PieceKind::Rook => 'r',
            &PieceKind::Knight => 'n',
            &PieceKind::Bishop => 'b',
            &PieceKind::Queen => 'q',
            &PieceKind::King => 'k',
        }
    }

    pub fn from_char(c: char) -> PieceKind {
        match c.to_ascii_lowercase() {
            'p' => PieceKind::Pawn,
            'r' => PieceKind::Rook,
            'n' => PieceKind::Knight,
            'b' => PieceKind::Bishop,
            'q' => PieceKind::Queen,
            'k' => PieceKind::King,
            _ => PieceKind::None,
        }
    }

    pub fn score(&self) -> i64 {
        match self {
            &PieceKind::None => 0,
            &PieceKind::Pawn => 1,
            &PieceKind::Rook => 5,
            &PieceKind::Knight => 3,
            &PieceKind::Bishop => 3,
            &PieceKind::Queen => 10,
            &PieceKind::King => 64 * 10 + 1,
        }
    }
}

impl PieceColor {
    pub fn opponent(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}


impl Piece {
    pub fn none() -> Piece {
        Piece {
            kind: PieceKind::None,
            color: PieceColor::White,
        }
    }

    pub fn new(kind: PieceKind, color: PieceColor) -> Piece {
        Piece {
            kind: kind,
            color: color,
        }
    }

    pub fn is_none(&self) -> bool {
        self.kind == PieceKind::None
    }

    pub fn to_char(&self) -> char {
        let c = self.kind.to_char();
        if self.color == PieceColor::White {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }

    pub fn from_char(c: char) -> Piece {
        Piece::new(PieceKind::from_char(c), if c.is_ascii_uppercase() { PieceColor::White } else { PieceColor::Black })
    }
}




impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}


impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceColor::White => write!(f, "White"),
            PieceColor::Black => write!(f, "Black"),
        }
        
    }
}
