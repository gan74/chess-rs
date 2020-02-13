use crate::board::*;
use crate::piece::*;
use crate::pos::*;
use crate::moves::*;

use std::io;
use std::str::FromStr;

use rand::{thread_rng, Rng};

pub trait PlayerController {
    fn play(&self, color: Color, board: &Board) -> Option<Move>;
}




pub struct Player();

impl Player {
    pub fn new_controller() -> Box<dyn PlayerController> {
        Box::new(Player{})
    }
}

impl PlayerController for Player {
    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        println!("{}", board);
        println!("{}'s turn:", color);
        loop {
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Some(m) = Move::from_str(&input).ok() {
                        if let Some(p) = board.piece_at(m.0) {
                            if p.color == color {
                                return Some(m);
                            }
                        }  
                        println!("Move is invalid.");
                    } else {
                        println!("Move could not be parsed.");
                    }
                }
                
                Err(_) => ()
            }
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
    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        let mut moves = Vec::new();

        let pieces = board.pieces(color);
        let enemy_king = board.king_pos(color.inverse()).unwrap_or(Pos::from_index(0));

        for i in 0..64 {
            let src = Pos::from_index(i);
            if pieces.piece_at(src) {
                let possible_dst = possible_moves(board, src);
                for i in 0..64 {
                    let dst = Pos::from_index(i);
                    if possible_dst.piece_at(dst) {
                        if self.check_mat && dst == enemy_king {
                            return Some(Move(src, dst));
                        }
                        moves.push(Move(src, dst));
                    }
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
    random: RandomAI
}

impl CaptureAI {
    pub fn new() -> CaptureAI {
        CaptureAI {
            random: RandomAI::new(false)
        }
    }

    pub fn new_controller() -> Box<dyn PlayerController> {
        Box::new(CaptureAI::new())
    }
}

impl PlayerController for CaptureAI {
    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        let mut best_score = -1;
        let mut best_move = None;

        let enemy_color = color.inverse();
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
                                if best_score < score {
                                    best_move = Some(Move(src, dst));
                                    best_score = score;
                                }
                            }
                        }
                    }
                }
            }
        }

        match best_move {
            None => self.random.play(color, board),
            m => m
        }
    }
}
