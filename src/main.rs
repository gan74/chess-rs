#![allow(dead_code)]

mod pos;
mod board;
mod piece;
mod moves;

use board::*;
use piece::*;
use pos::*;
use moves::*;

fn main() {
    let b = Board::new();
    println!("{}", b);
    println!("\n{}", b.pieces(Color::Black));
    println!("\n{}", b.pieces(Color::White));
    println!("\n{}", moves(&b, Pos::new(5, 1)));
}
