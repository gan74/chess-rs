use crate::piece::*;
use crate::pos::*;
use crate::moves::*;

use std::fmt;
use std::io;

use std::io::Write;
use std::cell::RefCell;



#[derive(Clone)]
pub struct Board {
    board: [Piece; 64],

    to_move: Color,

    kings: [Pos; 2],
}


impl Board {
    pub fn empty() -> Board {
        Board {
            board: [Piece::none(); 64],
            to_move: Color::White,
            kings: [Pos::new(0, 0); 2],
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::empty();

        let mut parts = fen.split_whitespace();

        let mut file: i32 = 0;
        let mut rank: i32 = 7;
        for c in parts.next().expect("Invalid FEN string").chars() {
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

        for c in parts.next().expect("Invalid FEN string").chars() {
            match c {
                ' ' => (),
                'w' => board.to_move = Color::White,
                'b' => board.to_move = Color::Black,
                _ => break,
            }
        }

        let cloned = board.clone();
        let move_set = generate_pseudo_legal_moves(&cloned);
        board.update_from_move_set(&move_set);
        board
    }

    pub fn to_move(&self) -> Color {
        self.to_move
    }

    #[inline(always)]
    pub fn piece_at(&self, pos: Pos) -> Piece {
        self.board[pos.index()]
    }

    pub fn all_pieces(&self) -> PieceIterator {
        PieceIterator {
            board: self,
            index: 0
        }
    }

    pub fn king_pos(&self, color: Color) -> Pos {
        self.kings[color.index()]
    }

    pub fn has_king(&self, color: Color) -> bool {
        self.piece_at(self.king_pos(color)) == Piece::new(PieceKind::King, color)
    }

    pub fn with_move(&self, mov: Move) -> Board {
        debug_assert!(std::ptr::eq(mov.parent_board(), self));
        debug_assert!(self.board[mov.src.index()].color == self.to_move);

        let mut board = self.clone();
        board.board[mov.dst.index()] = board.board[mov.src.index()];
        board.board[mov.src.index()] = Piece::none();
        board.to_move = board.to_move.opponent();
        board.update_from_move_set(mov.parent_move_set());
        board
    }

    pub fn san<T: Write>(&self, writer: &mut T, mov: Move) -> io::Result<()> {
        let piece = self.piece_at(mov.src);
        let capture = if self.piece_at(mov.dst).is_none() { "" } else { "x" };
        if piece.kind == PieceKind::Pawn {
            write!(writer, "{}{}{}", mov.src, capture, mov.dst)
        } else {
            write!(writer, "{}{}{}{}", piece.kind.to_char().to_ascii_uppercase(), mov.src, capture, mov.dst)
        }
    }




    fn update_from_move_set(&mut self, move_set: &MoveSet) {
        self.update_king_positions();
    }

    fn update_king_positions(&mut self) {
        for i in 0..64 {
            let p = self.board[i];
            if p.kind == PieceKind::King {
                self.kings[p.color.index()] = Pos::from_index(i);
            }
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
            Some((pos, self.board.piece_at(pos)))
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
                let piece = self.piece_at(Pos::new(x, row)).to_char();
                write!(f, " {}", piece)?;
            }
            write!(f, " |{}\n", row + 1)?;
        }
        write!(f, " +-----------------+\n")?;
        write!(f, "   a b c d e f g h\n")
    }
}
