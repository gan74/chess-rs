#![allow(dead_code)]

mod pos;
mod board;
mod piece;

use board::*;
use piece::*;

fn main() {
    let b = Board::new();
    println!("{}", b);
}
