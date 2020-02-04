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
		let pieces: [Piece; 8] = [Piece::Rook, Piece::Knight, Piece::Bishop, Piece::King, Piece::Queen, Piece::Bishop, Piece::Knight, Piece::Rook];
		for x in 0..8 {
			b.board[Pos::new(x, 0).index()] = pieces[x].colored(Color::Black);
			b.board[Pos::new(x, 1).index()] = Piece::Pawn.colored(Color::Black);

			b.board[Pos::new(x, 7).index()] = pieces[7 - x].colored(Color::White);
			b.board[Pos::new(x, 6).index()] = Piece::Pawn.colored(Color::White);
		}
		b
	}

	pub fn piece_at(&self, pos: Pos) -> ColoredPiece {
		self.board[pos.index()]
	}
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   a b c d e f g h\n")?;
        write!(f, " +-----------------+\n")?;
        for y in 0..8  {
        	write!(f, "{}|", 8 - y)?;
        	for x in 0..8  {
        		write!(f, " {}", self.piece_at(Pos::new(x, y)).char_for_piece())?;
        	}
        	write!(f, " |{}\n", 8 - y)?;
        }
        write!(f, " +-----------------+\n")?;
        write!(f, "   a b c d e f g h\n")
    }
}