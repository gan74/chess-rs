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
    let b = Board::test_board();
    println!("{}", b);
    println!("\n{}", b.pieces(Color::Black));
    println!("\n{}", b.pieces(Color::White));
	
	for i in 0..8 {
		println!("\n{}", moves(&b, Pos::new(i, 0)));
	}
	
}
