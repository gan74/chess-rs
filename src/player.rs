use crate::board::*;
use crate::piece::*;
use crate::moves::*;
use crate::pos::*;

use std::cmp;
use std::io;
use std::str::FromStr;

use rand::{thread_rng, Rng};




pub trait PlayerController {
    fn name(&self) -> String;
    fn play(&self, color: PieceColor, board: &Board) -> Option<Move>;
}



pub struct HumanPlayer();

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        HumanPlayer {
        }
    }
}

impl PlayerController for HumanPlayer {
    fn name(&self) -> String {
        "Human player".to_string()
    }

    fn play(&self, color: PieceColor, board: &Board) -> Option<Move> {
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
}

impl RandomAI {
    pub fn new() -> RandomAI {
        RandomAI {
        }
    }
}

impl PlayerController for RandomAI {
    fn name(&self) -> String {
        "Random".to_string()
    }

    fn play(&self, color: PieceColor, board: &Board) -> Option<Move> {
        let moves = generate_moves(board, color);

        let enemy_king_pos = board.king_pos(color.opponent()).unwrap_or(Pos::from_index(0));

        let mut total = 0;
        for mask in moves.move_masks() {
            if mask.dst.contains(enemy_king_pos) {
                return Some(Move(mask.src, enemy_king_pos));
            }
            total += mask.dst.bit_count();
        }

        match total {
            0 => None,
            l => {
                let mut rng = thread_rng();
                let mut index = rng.gen_range(0, l);
                for mask in moves.move_masks() {
                    let bits = mask.dst.bit_count();
                    if bits <= index {
                        index -= bits;
                        continue;
                    } 

                    return Some(Move(mask.src, mask.dst.set_positions().nth(index).unwrap()))
                }
                panic!("Invalid move index");
            }
        }
    }
}



