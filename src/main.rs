#![allow(dead_code)]

extern crate rand;
extern crate indicatif;

mod pos;
mod moves;
mod piece;
mod board;
mod bitboard;
mod player;
mod elo;

use crate::board::*;
use crate::bitboard::*;
use crate::piece::*;
use crate::moves::*;
use crate::pos::*;
use crate::elo::*;
use crate::player::*;


use std::str::FromStr;
use std::cmp;

use std::time::{Instant, Duration};
use rand::{thread_rng, Rng};
use indicatif::ProgressIterator;




const GAMES: usize = 50000;

fn gen_player_indexes(player_count: usize) -> (usize, usize) {
    assert!(player_count > 1);
    let mut rng = thread_rng();
    let i = rng.gen_range(0, player_count);
    let j = rng.gen_range(0, player_count - 1);

    if j == i {
        (i, player_count - 1)
    } else {
        (i, j)
    }
}

fn per_second(n: usize, time: Duration) -> f64 {
    (n as f64 / time.as_millis() as f64) * 1000.0
}

fn main() {
    let mut players = Vec::new();
    players.push(EloPlayer::new(RandomAI::new()));
    players.push(EloPlayer::new(RandomAI::new()));

    let start = Instant::now();

    println!("Simulating:");

    let mut moves = 0;
    for _ in (0..(GAMES / 1000)).progress() {
        for _ in 0..1000 {
            let (a, b) = gen_player_indexes(players.len());
            let (first, second) = (cmp::min(a, b), cmp::max(a, b));
            assert!(a != b);

            let (pa, pb) = players.split_at_mut(second);
            let pa: &mut EloPlayer = &mut pa[first];
            let pb: &mut EloPlayer = &mut pb[0];
            moves += pa.play_once(pb);
        }
    }

    let end = Instant::now();
    let time = end.duration_since(start);

    println!("\n{} games played in {:?} ({} g/s)", GAMES, time, per_second(GAMES, time).round() as i64);
    println!("{} moves played ({} m/s)", moves, per_second(moves, time).round() as i64);

    let mut total = 0;
    for player in players {
        println!("\n{}", player.name());
        println!("  elo: {}", player.elo_score().round() as i64);
        println!("  games: (w: {}, l: {}, d: {})", player.victories, player.loses, player.draws);
        total += player.victories + player.loses + player.draws;
    }
    assert!(total == GAMES * 2);
}