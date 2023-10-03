use crate::piece::*;
use crate::pos::*;

use std::fmt;
use std::io;

use std::io::Write;



#[derive(Clone)]
pub struct Board {
    board: [Piece; 64]
}


impl Board {
    pub fn empty() -> Board {
        Board {
            board: [Piece::none(); 64]
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::empty();

        let mut file: i32 = 0;
        let mut rank: i32 = 7;
        for c in fen.chars() {
            match c {
                '/' => {
                    file = 0;
                    rank -= 1;
                },
                c if c.is_ascii_digit() => {
                    file += c.to_digit(10).unwrap() as i32;
                },
                c => {
                    board.board[Pos::new(file, rank).index()] = Piece::from_char(c);
                    file += 1;
                },
            }
        }

        board
    }


    #[inline(always)]
    pub fn piece_at(&self, pos: Pos) -> Option<Piece> {
        match self.board[pos.index()] {
            p if !p.is_none() => Some(p),
            _ => None
        }
    }

    pub fn pieces(&self) -> PieceIterator {
        PieceIterator {
            board: self,
            index: 0
        }
    }

    pub fn has_king(&self, color: PieceColor) -> bool {
       self.king_pos(color).is_some()
    }

    pub fn king_pos(&self, color: PieceColor) -> Option<Pos> {
        for i in 0..64 {
            if self.board[i] == Piece::new(PieceKind::King, color) {
                return Some(Pos::from_index(i))
            }
        }
        None
    }

    pub fn with_move(&self, mov: Move) -> Board {
        let mut board = self.clone();
        board.board[mov.1.index()] = board.board[mov.0.index()];
        board.board[mov.0.index()] = Piece::none();
        board
    }

    pub fn san<T: Write>(&self, writer: &mut T, mov: Move) -> io::Result<()> {
        let piece = self.piece_at(mov.0).unwrap();
        let capture = if self.piece_at(mov.1).is_some() { "x" } else { "" };
        if piece.kind == PieceKind::Pawn {
            write!(writer, "{}{}{}", mov.0, capture, mov.1)
        } else {
            write!(writer, "{}{}{}{}", piece.kind.to_char().to_ascii_uppercase(), mov.0, capture, mov.1)
        }
    }
}




pub struct PieceIterator<'a> {
    board: &'a Board,
    index: usize,
}

impl<'a> Iterator for PieceIterator<'a> {
    type Item = (Pos, Piece);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 64 && self.board.piece_at(Pos::from_index(self.index)).is_none() {
            self.index += 1;
        }
        if self.index < 64 {
            let pos = Pos::from_index(self.index);
            self.index += 1;
            Some((pos, self.board.piece_at(pos).unwrap()))
        } else {
            None
        }
    }
}







impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   a b c d e f g h\n")?;
        write!(f, " +-----------------+\n")?;
        for y in 0..8  {
            let row = 7 - y;
            write!(f, "{}|", row + 1)?;
            for x in 0..8  {
                let piece = self.piece_at(Pos::new(x, row)).unwrap_or(Piece::none()).to_char();
                write!(f, " {}", piece)?;
            }
            write!(f, " |{}\n", row + 1)?;
        }
        write!(f, " +-----------------+\n")?;
        write!(f, "   a b c d e f g h\n")
    }
}
