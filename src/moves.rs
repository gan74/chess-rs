use crate::board::*;
use crate::piece::*;
use crate::pos::*;

use std::cmp;

pub struct PossibleMoveIterator<'a> {
    board: &'a Board,
    allies: BitBoard,
    enemies: BitBoard,
    dst_board: BitBoard,
    index: usize
}

impl<'a> PossibleMoveIterator<'a> {
    pub fn new(board: &Board, color: Color) -> PossibleMoveIterator {
        let allies = board.pieces(color);
        let enemies = board.pieces(color.inverse());
        let mut it = PossibleMoveIterator {
            board: board,
            allies: allies,
            enemies: enemies,
            dst_board: BitBoard::empty(),
            index: 0
        };
        it.recompute_dst();
        it
    }

    #[inline(always)]
    fn at_end(&self) -> bool {
        self.index == 64 * 64
    }

    #[inline(always)]
    fn advance(&mut self) {
        debug_assert!(!self.at_end());
        loop {
            self.index += 1;
            if !self.at_end() && self.index % 64 == 0 {
                self.recompute_dst();
                if self.dst_board.is_empty() {
                    continue;
                }
            }
            break;
        }
    }

    fn recompute_dst(&mut self) {
        let src_pos = Pos::from_index(self.index / 64);
        self.dst_board = {
            if self.allies.piece_at(src_pos) {
                match self.board.piece_at(src_pos) {
                    Some(piece) => possible_moves_internal(self.allies, self.enemies, piece, src_pos),
                    _ => unreachable!()
                }
            } else {
                BitBoard::empty()
            }
        };
    }

}

impl<'a> Iterator for PossibleMoveIterator<'a> {
    type Item = Move;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        while !self.at_end() {
            let dst_pos = Pos::from_index(self.index % 64);
            let src_pos = Pos::from_index(self.index / 64);
            if self.dst_board.piece_at(dst_pos) {
                self.advance();
                return Some(Move(src_pos, dst_pos))
            } else {
                self.advance();
            }
        }
        None
    }
}



pub fn possible_moves(board: &Board, pos: Pos) -> BitBoard {
    match board.piece_at(pos) {
        Some(piece) => possible_moves_for_piece(board, piece, pos),
        _ => BitBoard::empty()
    }
}

fn possible_moves_for_color(board: &Board, pos: Pos, color: Color) -> BitBoard {
    match board.piece_at(pos) {
        Some(piece) if piece.color == color => possible_moves_for_piece(board, piece, pos),
        _ => BitBoard::empty()
    }
}

fn possible_moves_for_piece(board: &Board, colored: ColoredPiece, pos: Pos) -> BitBoard {
    if colored.is_empty() {
        return BitBoard::empty();
    }

    let color = colored.color;
    let allies = board.pieces(color);
    let enemies = board.pieces(color.inverse());
    
    possible_moves_internal(allies, enemies, colored, pos)
}

fn possible_moves_internal(allies: BitBoard, enemies: BitBoard, colored: ColoredPiece, pos: Pos) -> BitBoard {
    if colored.is_empty() {
        return BitBoard::empty();
    }

    let color = colored.color;

    let col = pos.col();
    let row = pos.row();
    
    let moves = match colored.piece {
        Piece::Empty => {
            BitBoard::empty()
        }

        Piece::Pawn => {
            // TODO en passant
            let (mut dir, start_row) : (isize, usize) = if color == Color::Black {
                (-1, 6)
            } else {
                (1, 1)
            };

            if row == start_row {
                dir = dir * 2;
            }

            let dst_row = cmp::min(cmp::max(row as isize + dir, 0), 7) as usize;
            let mut pawn = path(enemies, allies, pos, col, dst_row, false);

            // captures
            {
                let dst_row = cmp::min(cmp::max(row as isize + dir.signum(), 0), 7) as usize;
                let left_col = cmp::max((col as isize) - 1, 0) as usize;
                let right_col = cmp::min(col + 1, 7);
                if enemies.piece_at(Pos::new(left_col, dst_row)) {
                    pawn.add(Pos::new(left_col, dst_row));
                }
                if enemies.piece_at(Pos::new(right_col, dst_row)) {
                    pawn.add(Pos::new(right_col, dst_row));
                }
            }

            pawn
        }

        Piece::Rook => {
            let n = path(enemies, allies, pos, col, 7, true);
            let s = path(enemies, allies, pos, col, 0, true);
            let e = path(enemies, allies, pos, 7, row, true);
            let w = path(enemies, allies, pos, 0, row, true);
            n.with_board(s).with_board(w).with_board(e)
        }
        
        Piece::Bishop => {
            let diff = cmp::min(col, row);
            let a = path(enemies, allies, pos, col - diff, row - diff, true);
            
            let diff = cmp::min(7 - col, row);
            let b = path(enemies, allies, pos, col + diff, row - diff, true);
            
            let diff = cmp::min(col, 7 - row);
            let c = path(enemies, allies, pos, col - diff, row + diff, true);
            
            let diff = cmp::min(7 - col, 7 - row);
            let d = path(enemies, allies, pos, col + diff, row + diff, true);
            
            a.with_board(b).with_board(c).with_board(d)
        }
        
        Piece::Queen => {
            possible_moves_internal(allies, enemies, Piece::Rook.colored(color), pos)
            .with_board(possible_moves_internal(allies, enemies, Piece::Bishop.colored(color), pos))
        }
        
        Piece::Knight => {
            let mut knight = BitBoard::empty();
            for offset in &[(2, 1), (-2, 1), (1, 2), (1, -2)] {
                for mul in &[-1, 1] {
                    let col = (col as isize) + (offset.0 * mul) as isize;
                    let row = (row as isize) + (offset.1 * mul) as isize;
                    if is_pos_valid(col, row) {
                        let p = Pos::new(col as usize, row as usize);
                        if !allies.piece_at(p) {
                            knight.add(p);
                        }
                    }
                }
            }
            knight
        }
        
        Piece::King => {
            let mut king = BitBoard::empty();
            for x in -1..2 {
                let col = (col as isize) + x;
                for y in -1..2 {
                    let row = (row as isize) + y;
                    if is_pos_valid(col, row) {
                        let p = Pos::new(col as usize, row as usize);
                        if !allies.piece_at(p) {
                            king.add(p);
                        }
                    }
                }
            }
            king
        }
    };

    debug_assert!(!moves.piece_at(pos));

    moves
}


fn is_pos_valid(col: isize, row: isize) -> bool {
    is_valid(col) && is_valid(row)
}

fn is_valid(w: isize) -> bool {
    w >= 0 && w < 8
}

fn path(enemies: BitBoard, allies: BitBoard, start: Pos, end_col: usize, end_row: usize, can_capture: bool) -> BitBoard {
    let mut d_col = (end_col as isize) - (start.col() as isize);
    let mut d_row = (end_row as isize) - (start.row() as isize);
    let s_col = d_col.signum();
    let s_row = d_row.signum();
    
    
    let mut board = BitBoard::empty();
    
        
    let mut col = start.col() as isize;
    let mut row = start.row() as isize;
    loop {
        if d_col.abs() == d_row.abs() {
            if d_col == 0 {
                break;
            }
            
            col += s_col;
            row += s_row;
            
            if !is_pos_valid(col, row) {
                break;
            }
            
            d_col -= s_col;
            d_row -= s_row;
        } else if d_col.abs() > d_row.abs() {
            col += s_col;
            
            if !is_valid(col) {
                break;
            }
            
            d_col -= s_col;
        } else {
            row += s_row;
            
            if !is_valid(row) {
                break;
            }
            
            d_row -= s_row;
        }
        
        debug_assert!(is_pos_valid(col, row));
        
        let p = Pos::new(col as usize, row as usize);
        if allies.piece_at(p) || (!can_capture && enemies.piece_at(p)) {
            break;
        }

        board.add(p);
        if enemies.piece_at(p) {
            break;
        }
    }
    board
}

