use std::vec::Vec;

use crate::piece::*;
use crate::pos::*;

use std::fmt;

#[derive(Clone, Copy)]
pub struct Board {
	board: [ColoredPiece; 64]
}

#[derive(Debug, Clone, Copy)]
pub struct BitBoard {
	board: u64
}


impl Board {
	pub fn empty() -> Board {
		Board {
			board: [ColoredPiece{piece: Piece::Empty, color: Color::Black}; 64]
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

	pub fn piece_at(&self, pos: Pos) -> ColoredPiece {
		self.board[pos.index()]
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
}



impl BitBoard {
	pub fn empty() -> BitBoard {
		BitBoard {
			board: 0
		}
	}

	pub fn piece_at(&self, pos: Pos) -> bool {
		(self.board >> pos.index()) & 1u64 == 1
	}


	pub fn add(&mut self, pos: Pos) {
		self.board = self.with(pos).board;
	}

	pub fn with(&self, pos: Pos) -> BitBoard {
		let mask = 1u64 << pos.index();
		BitBoard {
			board: self.board | mask
		}
	}


	pub fn add_board(&mut self, board: BitBoard) {
		self.board = self.with_board(board).board;
	}

	pub fn with_board(&self, board: BitBoard) -> BitBoard {
		BitBoard {
			board: self.board | board.board
		}
	}


	pub fn shift_x(&mut self, s: isize) {
		self.board = self.x_shifted(s).board;
	}

	pub fn x_shifted(&self, s: isize) -> BitBoard {
		BitBoard {
			board: if s > 0 {
				self.board << (s * 8)
			} else {
				self.board >> (-s * 8)
			}
		}
	}


	pub fn intersect(&mut self, board: BitBoard) {
		self.board = self.intersection(board).board
	}

	pub fn intersection(&self, board: BitBoard) -> BitBoard {
		BitBoard {
			board: self.board & board.board
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
        		let piece = self.piece_at(Pos::new(x, row)).char_for_piece();
        		write!(f, " {}", piece)?;
        	}
        	write!(f, " |{}\n", row + 1)?;
        }
        write!(f, " +-----------------+\n")?;
        write!(f, "   a b c d e f g h\n")
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   a b c d e f g h\n")?;
        write!(f, " +-----------------+\n")?;
        for y in 0..8  {
        	let row = 7 - y;
        	write!(f, "{}|", row + 1)?;
        	for x in 0..8  {
        		let bit = if self.piece_at(Pos::new(x, row)) {
        			1
        		} else {
        			0
        		};
        		write!(f, " {}", bit)?;
        	}
        	write!(f, " |{}\n", row + 1)?;
        }
        write!(f, " +-----------------+\n")?;
        write!(f, "   a b c d e f g h\n")
    }
}