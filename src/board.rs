use crate::piece::*;
use crate::pos::*;
use crate::moves::*;
use crate::bitboard::*;

use std::fmt;
use std::io;

use std::io::Write;
use std::cell::RefCell;



#[derive(Clone)]
pub struct Board {
    board: [Piece; 64],

    kings: [Pos; 2],
    pieces: [BitBoard; 2],

    to_move: Color,
}


impl Board {
    pub fn empty() -> Board {
        Board {
            board: [Piece::none(); 64],

            kings: [Pos::new(0, 0); 2],
            pieces: [BitBoard::empty(); 2],

            to_move: Color::White,
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

        for i in 0..64 {
            let p = board.board[i];
            if !p.is_none() {
                let color_index = p.color.index();
                let pos = Pos::from_index(i);
                board.pieces[color_index] += pos;
                if p.kind == PieceKind::King {
                    board.kings[color_index] = pos;
                }
            }
        }

        board
    }


    pub fn to_move(&self) -> Color {
        self.to_move
    }

    #[inline(always)]
    pub fn piece_at(&self, pos: Pos) -> Piece {
        self.board[pos.index()]
    }

    pub fn all_pieces(&self) -> impl Iterator<Item = (Pos, Piece)> + '_ {
        self.board.iter().enumerate().filter(|e| !e.1.is_none()).map(|e| (Pos::from_index(e.0), *e.1))
    }

    pub fn pieces_for(&self, color: Color) -> BitBoard {
        self.pieces[color.index()]
    }

    pub fn king_pos(&self, color: Color) -> Pos {
        self.kings[color.index()]
    }

    pub fn has_king(&self, color: Color) -> bool {
        self.piece_at(self.king_pos(color)) == Piece::new(PieceKind::King, color)
    }




    pub fn play(&self, mov: Move) -> Board {
        debug_assert!(std::ptr::eq(mov.parent_board(), self));
        debug_assert!(!self.board[mov.src.index()].is_none());
        debug_assert!(self.board[mov.src.index()].color == self.to_move);

        let mut board = self.clone();
        {
            let color_index = board.to_move.index();

            // Fix king pos
            if board.board[mov.src.index()].kind == PieceKind::King {
                board.kings[color_index] = mov.dst;
            }

            // Move piece on board
            board.board[mov.dst.index()] = board.board[mov.src.index()];
            board.board[mov.src.index()] = Piece::none();

            // Move piece in bitboards
            board.pieces[color_index] -= mov.src;
            board.pieces[color_index] += mov.dst;
            board.pieces[1 - color_index] -= mov.dst;
            debug_assert!(board.pieces[color_index].bit_count() == self.pieces[color_index].bit_count());
            debug_assert!(board.pieces[1 - color_index].bit_count() <= self.pieces[1 - color_index].bit_count());

            // Change player
            board.to_move = board.to_move.opponent();
        }
        board
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
