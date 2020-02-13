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
        if let Some(m) = player.play(color, &board) {
            color = color.inverse();
        } else {
            break;
        }
    }
}
