use crate::board::*;
use crate::piece::*;
use crate::pos::*;

use std::io;
use std::str::FromStr;

pub trait PlayerController {
    fn name(&self) -> String;
    fn play(&self, color: Color, board: &Board) -> Option<Move>;
}




pub struct Player();

impl Player {
    pub fn new() -> Player {
        Player {
        }
    }
}

impl PlayerController for Player {
    fn name(&self) -> String {
        "Human player".to_string()
    }

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