use crate::board::*;
use crate::piece::*;
use crate::pos::*;
use crate::moves::*;

use std::io;
use std::str::FromStr;

pub trait PlayerController {
    fn play(&self, color: Color, board: &Board) -> Option<Move>;
}



pub struct Player();

impl PlayerController for Player {
    fn play(&self, color: Color, board: &Board) -> Option<Move> {
        println!("{}", board);
        println!("{}'s turn:", color);
        loop {
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Some(m) = Move::from_str(&input).ok() {
                        if board.piece_at(m.0).color == color {
                            if possible_moves(board, m.0).piece_at(m.1) {
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