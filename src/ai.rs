use crate::board::*;
use crate::piece::*;
use crate::pos::*;
use crate::moves::*;
use crate::player::*;

use rand::{thread_rng, Rng};



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
        for src in board.pieces(color).iter() {
            for dst in possible_moves(board, src).iter() {
                return Some(Move(src, dst));
            }
        }
        None
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

        for src in board.pieces(color).iter() {
            for dst in possible_moves(board, src).iter() {
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

        for src in board.pieces(color).iter() {
            for dst in possible_moves(board, src).iter() {
                let score = distance(enemy_king, dst);
                if score < best_score {
                    best_move = Some(Move(src, dst));
                    best_score = score;
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