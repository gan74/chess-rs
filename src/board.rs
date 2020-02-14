use crate::bitboard::*;
use crate::piece::*;
use crate::moves::*;
use crate::pos::*;

use std::fmt;

#[derive(Clone)]
pub struct Board {
    board: [ColoredPiece; 64]
}

impl Board {
    pub fn empty() -> Board {
        Board {
            board: [ColoredPiece::empty(); 64]
        }
    }

    pub fn new() -> Board {
        let mut b = Board::empty();
        let pieces: [Piece; 8] = [Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook];
        for x in 0..8 {
            b.board[Pos::new(x, 7).index()] = pieces[7 - x].colored(Color::Black);
            b.board[Pos::new(x, 6).index()] = Piece::Pawn.colored(Color::Black);

            b.board[Pos::new(x, 0).index()] = pieces[x].colored(Color::White);
            b.board[Pos::new(x, 1).index()] = Piece::Pawn.colored(Color::White);
        }
        b
    }
    
    pub fn test_board() -> Board {
        let mut b = Board::empty();
        let pieces: [Piece; 8] = [Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook];
        for x in 0..8 {
            b.board[Pos::new(x, 7).index()] = pieces[7 - x].colored(Color::Black);
            b.board[Pos::new(x, 6).index()] = Piece::Pawn.colored(Color::Black);
            b.board[Pos::new(x, 0).index()] = pieces[x].colored(Color::White);
        }
        b
    }



    pub fn possible_moves(&self, color: Color) -> PossibleMoveIterator {
        PossibleMoveIterator::new(self, color)
    }

    #[inline(always)]
    pub fn piece_at(&self, pos: Pos) -> Option<ColoredPiece> {
        match self.board[pos.index()] {
            p if !p.is_empty() => Some(p),
            _ => None
        }
    }



    pub fn pieces(&self, col: Color) -> BitBoard {
        let mut b = BitBoard::empty();
        for i in 0..64 {
            if !self.board[i].is_empty() && self.board[i].color == col {
                b.add(Pos::from_index(i));
            }
        }
        b
    }


    pub fn has_king(&self, color: Color) -> bool {
       self.king_pos(color).is_some()
    }

    pub fn king_pos(&self, color: Color) -> Option<Pos> {
        for i in 0..64 {
            let p = Pos::from_index(i);
            if let Some(k) = self.piece_at(p) {
                if k.color == color && k.piece == Piece::King {
                    return Some(p)
                }
            }
        }
        None
    }


    pub fn do_move(&mut self, m: Move) {
        self.board[m.1.index()] = self.board[m.0.index()];
        self.board[m.0.index()].piece = Piece::Empty;
    }

    pub fn moved(&self, m: Move) -> Board {
        let mut b = self.clone();
        b.do_move(m);
        b
    }

    pub fn try_move(&self, m: Move) -> Result<Board, ()> {
        if self.is_valid_move(m) {
            Ok(self.moved(m))
        } else {
            Err(())
        }
    }

    pub fn is_valid_move(&self, m: Move) -> bool {
        possible_moves(self, m.0).piece_at(m.1)
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
                let piece = self.piece_at(Pos::new(x, row)).unwrap_or(ColoredPiece::empty()).char_for_piece();
                write!(f, " {}", piece)?;
            }
            write!(f, " |{}\n", row + 1)?;
        }
        write!(f, " +-----------------+\n")?;
        write!(f, "   a b c d e f g h\n")
    }
}
