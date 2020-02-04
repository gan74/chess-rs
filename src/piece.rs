
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

#[derive(Debug, Clone, Copy)]
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
}

impl ColoredPiece {
	pub fn char_for_piece(&self) -> char {
		let c = self.piece.char_for_piece();
		match self.color {
			Color::Black => c,
			Color::White => c.to_uppercase().next().unwrap_or(Piece::Empty.char_for_piece())
		}
	}
}