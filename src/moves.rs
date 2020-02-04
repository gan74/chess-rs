use crate::board::*;
use crate::piece::*;
use crate::pos::*;

pub fn moves(board: &Board, pos: Pos) -> BitBoard {
	let piece = board.piece_at(pos);
	let col = piece.color;
	match piece.piece {
		Piece::Empty => {
			BitBoard::empty()
		}

		Piece::Pawn => {
			let mut pawn = BitBoard::empty().with(pos);

			let (dir, start) = if col == Color::Black {
				(-1, 6)
			} else {
				(1, 1)
			};

			pawn.shift_x(dir);
			if pos.row() == start {
				// TODO captures
				pawn.add_board(pawn.x_shifted(dir));
			}

			pawn
		}

		_ => {
			panic!("Unknown piece type!")
		}
	}
}

