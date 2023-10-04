use crate::pos::*;
use crate::board::*;
use crate::bitboard::*;
use crate::piece::*;

use std::fmt;



#[derive(Debug, Clone, Copy)]
pub struct MoveMask {
    pub dst: BitBoard,
    pub src: Pos,
}

#[derive(Clone)]
pub struct MoveSet<'a> {
    parent: &'a Board,

    masks: [MoveMask; 16],
    mask_count: usize,

    dst_positions: BitBoard,
}

#[derive(Clone, Copy)]
pub struct Move<'a> {
    parent: &'a MoveSet<'a>,

    pub src: Pos,
    pub dst: Pos,
}



impl<'a> MoveSet<'a> {
    fn new(board: &Board) -> MoveSet {
        let empty_move_mask = MoveMask {
            dst: BitBoard::empty(),
            src: Pos::new(0, 0),
        };
        MoveSet {
            parent: board,
            masks: [empty_move_mask; 16],
            mask_count: 0,
            dst_positions: BitBoard::empty(),
        }
    }



    pub fn move_masks(&self) -> &[MoveMask] {
        &self.masks[0..self.mask_count]
    }

    pub fn all_dst_positions(&self) -> BitBoard {
        self.dst_positions
    }

    pub fn is_empty(&self) -> bool {
        self.mask_count == 0
    }

    pub fn moves(&self) -> MoveIterator {
        MoveIterator {
            inner: self.moves_for_mask(0),
        }
    }

    pub fn moves_for_mask(&self, index: usize) -> MoveMaskIterator {
        debug_assert!(index < self.mask_count);
        MoveMaskIterator {
            parent: self,
            bits: self.masks[index].dst.set_positions(),
            mask_index: index,
        }
    }

    pub fn moves_ending_in<'b>(&'b self, pos: Pos) -> impl Iterator<Item = Move<'b>> {
        self.masks[0..self.mask_count].iter()
            .filter(move |mask| mask.dst.contains(pos))
            .map(move |mask| 
                Move {
                    parent: self,
                    src: mask.src,
                    dst: pos,
                }
            )
    }

    pub fn parent_board(&self) -> &Board {
        return self.parent
    }


    fn push(&mut self, mov: MoveMask) {
        debug_assert!(self.mask_count < self.masks.len());
        self.masks[self.mask_count] = mov;
        self.mask_count += 1;
        self.dst_positions += mov.dst;
    }
}


impl<'a> Move<'a> {
    pub fn parent_board(&self) -> &Board {
        return self.parent.parent
    }

    pub fn parent_move_set(&self) -> &MoveSet {
        return self.parent
    }

    pub fn san(&self) -> SanToken {
        let board = self.parent_board();
        let color = board.piece_at(self.src).color;
        let enemy_king_pos = board.king_pos(color.opponent());
        SanToken {
            kind: board.piece_at(self.src).kind,
            src: self.src,
            dst: self.dst,
            is_capture: !board.piece_at(self.dst).is_none(),
            is_check: generate_pseudo_legal_moves(&board).all_dst_positions().contains(enemy_king_pos),
        }
    }
}






#[derive(Clone, Copy)]
pub struct MoveMaskIterator<'a> {
    parent: &'a MoveSet<'a>,
    bits: BitBoardIterator,
    mask_index: usize,
}

#[derive(Clone, Copy)]
pub struct MoveIterator<'a> {
    inner: MoveMaskIterator<'a>,
}

impl<'a> Iterator for MoveMaskIterator<'a> {
    type Item = Move<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.bits.next().map(|dst| 
            Move {
                parent: self.parent,
                src: self.parent.masks[self.mask_index].src,
                dst: dst,
            }
        )
    }

    fn count(self) -> usize {
        self.bits.count()
    }
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = Move<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let m = self.inner.next();
        if m.is_none() {
            self.inner.mask_index += 1;
            if self.inner.mask_index < self.inner.parent.mask_count {
                self.inner.bits = self.inner.parent.masks[self.inner.mask_index].dst.set_positions();
                self.next()
            } else {
                None
            }
        } else {
            m
        }
    }


    fn count(self) -> usize {
        let mut total = self.inner.clone().count();
        for i in (self.inner.mask_index + 1)..self.inner.parent.mask_count {
            total += self.inner.parent.masks[i].dst.bit_count();
        }
        total
    }
}








pub fn generate_pseudo_legal_moves(board: &Board) -> MoveSet {
    generate_pseudo_legal_moves_for_color(board, board.to_move())
}



pub fn generate_pseudo_legal_moves_for_color(board: &Board, to_move: Color) -> MoveSet {
    let allies = board.pieces_for(to_move);
    let opponents = board.pieces_for(to_move.opponent());
    let all_pieces = allies + opponents;

    let mut moves = MoveSet::new(board);

    for base_pos in allies.set_positions() {
        debug_assert!(allies.contains(base_pos));
        let piece = board.piece_at(base_pos);
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
        match piece.kind {
            PieceKind::None => {},

            PieceKind::Pawn => {
                let (dir, moved) = match piece.color {
                    Color::White => ( 1, py != 1),
                    Color::Black => (-1, py != 6),
                };
                if let Some(pos) = Pos::try_new(px, py + dir) {
                    let adv = dst_board + pos;
                    dst_board += adv.shift_left().intersect(opponents);
                    dst_board += adv.shift_right().intersect(opponents);
                    
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









pub struct SanToken {
    kind: PieceKind,
    src: Pos,
    dst: Pos,

    is_capture: bool,
    is_check: bool,
}

impl SanToken {
    pub fn new() -> SanToken {
        SanToken {
            kind: PieceKind::King,
            src: Pos::new(0, 0),
            dst: Pos::new(0, 0),
            is_capture: false,
            is_check: false,
        }
    }
}

impl<'a> fmt::Display for SanToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.kind != PieceKind::Pawn {
            write!(f, "{}", self.kind.to_char().to_ascii_uppercase())?;
        }

        write!(f, "{}", self.src)?;

        if self.is_capture {
            write!(f, "x")?;
        }

        write!(f, "{}", self.dst)?;

        if self.is_check {
            write!(f, "+")?;
        }

        Ok(())
    }
}