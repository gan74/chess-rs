use crate::board::*;
use crate::piece::*;
use crate::pos::*;
use crate::moves::*;

use std::io;
use std::str::FromStr;

use rand::Rng;

pub trait PlayerController {
    fn play(&self, color: Color, board: &Board) -> Option<Move>;
}




pub struct Player();

impl Player {
    pub fn new() -> Box<dyn PlayerController> {
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


pub struct RandomAI();

impl RandomAI {
    pub fn new() -> Box<dyn PlayerController> {
        Box::new(RandomAI{})
    }
}

impl PlayerController for RandomAI {
    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        let mut moves = Vec::new();

        let pieces = board.pieces(color);
        for i in 0..64 {
            let src = Pos::from_index(i);
            if pieces.piece_at(src) {
                let possible_dst = possible_moves(board, src);
                for i in 0..64 {
                    let dst = Pos::from_index(i);
                    if possible_dst.piece_at(dst) {
                        moves.push(Move(src, dst));
                    }
                }
            }
        }

        match moves.len() {
            0 => None,
            l => {
                let mut rng = rand::thread_rng();
                Some(moves[rng.gen_range(0, l)])
            }
        }
    }
}
