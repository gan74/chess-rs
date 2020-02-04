
#[derive(Debug, Clone, Copy)]
pub enum Piece {
	Empty,
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
	Black,
	White
}

pub struct ColoredPiece {
	piece: Piece,
	color: Color
}

impl Piece {
	pub fn char_for_piece(&self) -> char {
		match self {
			Empty => ' ',
			Pawn => 'p',
			Rook => 'r',
			Knight => 'n',
			Bishop => 'b',
			Queen => 'q',
			King => 'k'
		}
	}
}

impl ColoredPiece {
	pub fn char_for_piece(&self) -> char {
		let c = self.piece.char_for_piece();
		match self.color {
			Black => c,
			White => c.to_uppercase().next().unwrap_or(' ')
		}
	}
}