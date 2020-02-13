use crate::board::*;
use crate::piece::*;
use crate::pos::*;
use crate::moves::*;
use crate::player::*;

use rand::{thread_rng, Rng};


const USE_ITERATOR: bool = true;

// http://tom7.org/chess/weak.pdf

pub struct FirstMoveAI {
}

impl FirstMoveAI {
    pub fn new_controller() -> Box<dyn PlayerController> {
        Box::new(FirstMoveAI{})
    }
}

impl PlayerController for FirstMoveAI {
    fn name(&self) -> String {
        "FirstMove".to_string()
    }

    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        if USE_ITERATOR {
            board.possible_moves(color).next()
        } else {
            let pieces = board.pieces(color);
            for i in 0..64 {
                let src = Pos::from_index(i);
                if pieces.piece_at(src) {
                    let possible_moves = possible_moves(board, src);

                    for i in 0..64 {
                        let dst = Pos::from_index(i);
                        if possible_moves.piece_at(dst) {
                            return Some(Move(src, dst));
                        }
                    }
                }
            }
            None
        }
    }
}




pub struct RandomAI {
    check_mat: bool
}

impl RandomAI {
    pub fn new(check_mat: bool) -> RandomAI {
        RandomAI {
            check_mat: check_mat
        }
    }

    pub fn new_controller() -> Box<dyn PlayerController> {
        Box::new(RandomAI::new(true))
    }
}

impl PlayerController for RandomAI {
    fn name(&self) -> String {
        "Random".to_string()
    }

    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        let moves = board.possible_moves(color).collect::<Vec<_>>();

        if self.check_mat {
            let enemy_king = board.king_pos(color.inverse()).unwrap_or(Pos::from_index(0));
            for m in &moves {
                if m.1 == enemy_king {
                    return Some(*m);
                }
            }
        }

        match moves.len() {
            0 => None,
            l => {
                let mut rng = thread_rng();
                Some(moves[rng.gen_range(0, l)])
            }
        }
    }
}





pub struct CaptureAI {
    fallback: Box<dyn PlayerController>
}

impl CaptureAI {
    pub fn new() -> CaptureAI {
        CaptureAI::new_with_fallback(SwarmAI::new_controller())
    }

    pub fn new_with_fallback(fallback: Box<dyn PlayerController>) -> CaptureAI {
        CaptureAI {
            fallback: fallback
        }
    }

    pub fn new_controller() -> Box<dyn PlayerController> {
        Box::new(CaptureAI::new())
    }
}

impl PlayerController for CaptureAI {
    fn name(&self) -> String {
        "Capture".to_string()
    }

    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        let mut capture_score = -1;
        let mut best_capture = None;

        let enemy_color = color.inverse();

        if USE_ITERATOR {
            for m in board.possible_moves(color) {
                match board.piece_at(m.1) {
                    Some(p) if p.color == enemy_color => {
                        let score = p.piece.score();
                        if capture_score < score {
                            best_capture = Some(m);
                            capture_score = score;
                        }
                    }

                    _ => ()
                }
            }
        } else {
            let pieces = board.pieces(color);

            for i in 0..64 {
                let src = Pos::from_index(i);
                if pieces.piece_at(src) {
                    let possible_moves = possible_moves(board, src);

                    for i in 0..64 {
                        let dst = Pos::from_index(i);
                        if possible_moves.piece_at(dst) {
                            if let Some(p) = board.piece_at(dst) {
                                if p.color == enemy_color {
                                    let score = p.piece.score();
                                    if capture_score < score {
                                        best_capture = Some(Move(src, dst));
                                        capture_score = score;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        match best_capture {
            None => self.fallback.play(color, board),
            m => m
        }
    }
}




pub struct SwarmAI {
}

impl SwarmAI {
    pub fn new() -> SwarmAI {
        SwarmAI {
        }
    }

    pub fn new_controller() -> Box<dyn PlayerController> {
        Box::new(SwarmAI::new())
    }
}

impl PlayerController for SwarmAI {
    fn name(&self) -> String {
        "Swarm".to_string()
    }

    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        let mut best_score = 8 + 8 + 1;
        let mut best_move = None;

        let enemy_king = board.king_pos(color.inverse()).unwrap_or(Pos::from_index(0));

        if USE_ITERATOR {
            for m in board.possible_moves(color) {
                let score = distance(enemy_king, m.1);
                if score < best_score {
                    best_move = Some(m);
                    best_score = score;
                }
            }
        } else {
            let pieces = board.pieces(color);
            for i in 0..64 {
                let src = Pos::from_index(i);
                if pieces.piece_at(src) {
                    let possible_moves = possible_moves(board, src);

                    for i in 0..64 {
                        let dst = Pos::from_index(i);
                        if possible_moves.piece_at(dst) {
                            let score = distance(enemy_king, dst);
                            if score < best_score {
                                best_move = Some(Move(src, dst));
                                best_score = score;
                            }
                        }
                    }
                }
            }
        }
        
        best_move
    }
}



fn distance(a: Pos, b: Pos) -> i64 {
    let d_col = (a.col() as i64) - (b.col() as i64);
    let d_row = (a.row() as i64) - (b.row() as i64);
    d_col.abs() + d_row.abs()
}