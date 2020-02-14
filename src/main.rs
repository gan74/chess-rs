#![allow(dead_code)]

extern crate rand;

mod elo;
mod pos;
mod board;
mod bitboard;
mod piece;
mod moves;
mod player;
mod ai;

use elo::*;
use ai::*;

use std::time::Instant;
use std::cmp;

use rand::{thread_rng, Rng};

const GAMES: usize = 100000;

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

fn main() {
    let mut players = Vec::new();
    players.push(EloPlayer::new(RandomAI::new(true)));
    players.push(EloPlayer::new(FirstMoveAI::new()));
    players.push(EloPlayer::new(SwarmAI::new()));
    players.push(EloPlayer::new(CaptureAI::new()));

    let start = Instant::now();

    for _ in 0..GAMES {
        let (a, b) = gen_player_indexes(players.len());
        let (first, second) = (cmp::min(a, b), cmp::max(a, b));
        assert!(a != b);

        let (pa, pb) = players.split_at_mut(second);
        let pa: &mut EloPlayer = &mut pa[first];
        let pb: &mut EloPlayer = &mut pb[0];
        pa.play_once(pb);
    }

    let end = Instant::now();
    let time = end.duration_since(start);

    println!("{} games played in {:?} ({} g/s)", GAMES, time, ((GAMES as f64 / time.as_millis() as f64) * 1000.0).round() as i64);

    let mut total = 0;
    for player in players {
        println!("\n{}", player.name());
        println!("  elo: {}", player.elo_score().round() as i64);
        println!("  games: (w: {}, l: {}, d: {})", player.victories, player.loses, player.draws);
        total += player.victories + player.loses + player.draws;
    }
    assert!(total == GAMES * 2);
}
