use crate::pos::*;
use crate::board::*;
use crate::bitboard::*;
use crate::piece::*;


#[derive(Debug, Clone, Copy)]
pub struct MoveMask {
    pub dst: BitBoard,
    pub src: Pos,
}

#[derive(Debug, Clone)]
pub struct MoveSet {
    masks: [MoveMask; 32],
    move_count: usize,
}

impl MoveSet {
    pub fn new() -> MoveSet {
        let empty_move_mask = MoveMask {
            dst: BitBoard::empty(),
            src: Pos::new(0, 0),
        };
        MoveSet {
            masks: [empty_move_mask; 32],
            move_count: 0,
        }
    }


    pub fn push(&mut self, mov: MoveMask) {
        debug_assert!(self.move_count + 1 < self.masks.len());
        self.masks[self.move_count] = mov;
        self.move_count += 1;
    }

    pub fn move_masks(&self) -> &[MoveMask] {
        &self.masks[0..self.move_count]
    }

    pub fn dst_positions(&self) -> BitBoard {
        let mut dst = BitBoard::empty();
        for mask in self.move_masks() {
            dst += mask.dst;
        }
        dst
    }
}








pub fn generate_moves(board: &Board, color: PieceColor) -> MoveSet {
    let allies = bitboard_for_color(board, color);
    let enemies = bitboard_for_color(board, color.opponent());
    let all_pieces = allies + enemies;

    let mut moves = MoveSet::new();
    for p in board.pieces().filter(|p| p.1.color == color) {
        let base_pos = p.0;
        let px = base_pos.col();
        let py = base_pos.row();

        let move_bishop = |dst: &mut BitBoard| {
            let mut add_pos = |ox, oy| {
                if let Some(pos) = Pos::try_new(px - ox, py - oy) {
                    debug_assert!(!dst.contains(pos));
                    *dst += pos;
                    return !all_pieces.contains(pos);
                }
                false
            };

            for i in 1..8 {
                if !add_pos(-i, -i) {
                    break
                }
            }
            for i in 1..8 {
                if !add_pos(-i, i) {
                    break
                }
            }

            for i in 1..8 {
                if !add_pos(i, -i) {
                    break
                }
            }
            for i in 1..8 {
                if !add_pos(i, i) {
                    break
                }
            }
        };

        let move_rook = |dst: &mut BitBoard| {
            let mut add_pos = |x, y| {
                let pos = Pos::new(x, y);
                debug_assert!(!dst.contains(pos));
                *dst += pos;
                !all_pieces.contains(pos)
            };


            for x in (0..px).rev() {
                if !add_pos(x, py) {
                    break;
                }
            }
            for x in (px + 1)..8 {
                if !add_pos(x, py) {
                    break;
                }
            }

            for y in (0..py).rev() {
                if !add_pos(px, y) {
                    break;
                }
            }
            for y in (py + 1)..8 {
                if !add_pos(px, y) {
                    break;
                }
            }
        };


        let mut dst_board = BitBoard::empty();
        match p.1.kind {
            PieceKind::None => {},

            PieceKind::Pawn => {
                let (dir, moved) = match p.1.color {
                    PieceColor::White => ( 1, py != 1),
                    PieceColor::Black => (-1, py != 6),
                };
                if let Some(pos) = Pos::try_new(px, py + dir) {
                    let adv = dst_board + pos;
                    dst_board += adv.shift_left().intersect(enemies);
                    dst_board += adv.shift_right().intersect(enemies);
                    
                    if !all_pieces.contains(pos) {
                        dst_board += pos;
                        if !moved {
                            if let Some(pos) = Pos::try_new(px, py + dir + dir) {
                                if !all_pieces.contains(pos) {
                                    dst_board += pos;
                                }
                            }
                        }
                    }
                }
            },

            PieceKind::Bishop => { 
                move_bishop(&mut dst_board);
            },

            PieceKind::Rook => {
                move_rook(&mut dst_board);
            },

            PieceKind::Queen => {
                move_bishop(&mut dst_board);
                move_rook(&mut dst_board);
            },

            PieceKind::King => {
                for x in -1..=1 {
                    for y in -1..=1 {
                        if let Some(pos) = Pos::try_new(px + x, py + y) {
                            dst_board += pos;
                        }
                    }
                }
            },

            PieceKind::Knight => {
                let offsets = [
                    ( 2,  1), ( 1,  2), 
                    (-2,  1), (-1,  2), 
                    ( 2, -1), ( 1, -2), 
                    (-2, -1), (-1, -2), 
                ];
                for (x, y) in offsets {
                    if let Some(pos) = Pos::try_new(px + x, py + y) {
                        dst_board += pos;
                    }
                }
            },

        }

        dst_board -= allies;
        if !dst_board.is_empty() {
            moves.push(MoveMask {
                dst: dst_board,
                src: base_pos,
            })
        }
    }

    moves
}




fn bitboard_for_color(board: &Board, color: PieceColor) -> BitBoard {
    BitBoard::from(board.pieces().filter(|p| p.1.color == color).map(|p| p.0))
}