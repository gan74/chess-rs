#![allow(dead_code)]

extern crate rand;

mod pos;
mod board;
mod piece;
mod moves;
mod player;

use board::*;
use piece::*;
use player::*;

fn main() {
    let player_0 = RandomAI::new();
    let player_1 = RandomAI::new();
    let players = [player_0, player_1];
    
    let mut board = Board::new();
    let mut color = Color::White;
    loop {
        if !board.has_king(color) {
            break;
        }

        let player = match color {
            Color::Black => &players[1],
            Color::White => &players[0],
        };

        println!("{}", board);

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
    println!("{} lost!", color);
    println!("final board:\n{}", board);
}
