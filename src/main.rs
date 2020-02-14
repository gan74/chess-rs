#![allow(dead_code)]

extern crate rand;

mod pos;
mod board;
mod bitboard;
mod piece;
mod moves;
mod player;
mod ai;

use board::*;
use piece::*;
use player::*;
use ai::*;

use std::time::Instant;

fn play_once(players: &[Box<dyn PlayerController>; 2], max_moves: usize) -> Option<Color> {
    let mut board = Board::new();
    let mut color = Color::White;

    for _moves in 0..max_moves {
        if !board.has_king(color) {
            //println!("{}", _moves);
            break;
        }

        if let Some(m) = players[color.index()].play(color, &board) {
            match board.try_move(m) {
                Ok(b) => {
                    board = b;
                    color = color.inverse();
                }

                Err(_) => {
                    println!("Invalid move ({}).", color);
                    break;
                }
            }
        } else {
            break;
        }
    };

    let winner = color.inverse();

    /*println!("{} wins!", winner);
    println!("final board:\n{}", board);*/

    Some(winner)
}

fn main() {
    let players = [FirstMoveAI::new_controller(), FirstMoveAI::new_controller()];

    let mut total_games = 0;
    let mut draws = 0;
    let mut victories = [0, 0];

    let start = Instant::now();
    while total_games < 10000 {
        if let Some(winner) = play_once(&players, 100) {
            victories[winner.index()] += 1;
        } else {
            draws += 1;
        }
        total_games += 1
    }
    let end = Instant::now();
    let time = end.duration_since(start);
    
    if victories[0] > victories[1] {
        println!("{} Wins!", players[0].name());
    } else if victories[1] > victories[0] {
        println!("{} Wins!", players[1].name());
    } else {
        println!("Draw!");
    }
    println!("total games: {}", total_games);
    println!("  draws: {}", draws);
    println!("  {} victories: {}", players[0].name(), victories[0]);
    println!("  {} victories: {}", players[1].name(), victories[1]);
    println!("  total time: {:?} ({} g/s)", time, ((total_games as f64 / time.as_millis() as f64) * 1000.0).round() as i64);
}
