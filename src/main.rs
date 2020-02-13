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

use std::time::Instant;

fn play_once(players: &[Box<dyn PlayerController>; 2], max_moves: usize) -> Option<Color> {
    let mut board = Board::new();
    let mut color = Color::White;

    for _ in 0..max_moves {
        if !board.has_king(color) {
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
    Some(winner)
}

fn main() {
    let players = [CaptureAI::new_controller(), RandomAI::new_controller()];

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
    
    println!("total games: {}", total_games);
    println!("  draws: {}", draws);
    println!("  white victories: {}", victories[Color::White.index()]);
    println!("  black victories: {}", victories[Color::Black.index()]);
    println!("  total time: {:?} ({} g/s)", time, ((total_games as f64 / time.as_millis() as f64) * 1000.0).round() as i64);
}
