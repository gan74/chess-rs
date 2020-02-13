#![allow(dead_code)]

mod pos;
mod board;
mod piece;
mod moves;
mod player;

use board::*;
use piece::*;
use pos::*;
use moves::*;
use player::*;

fn main() {
    let player = Player{};
    
    let mut board = Board::new();
    let mut color = Color::White;
    loop {
        if !board.has_king(color) {
            println!("{} lost!", color);
            break;
        }
        
        if let Some(m) = player.play(color, &board) {
            match board.try_move(m) {
                Ok(b) => {
                    board = b;
                    color = color.inverse();
                }

                Err(_) => {
                    println!("Invalid move.");
                }
            }
        } else {
            break;
        }
    }
}
