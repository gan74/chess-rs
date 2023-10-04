#![allow(dead_code)]
#![allow(unused_imports)]

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

use rayon::prelude::*;

use std::fmt;
use std::cmp;
use std::io;

use std::str::FromStr;
use std::io::Write;

use std::time::{Instant, Duration};
use rand::{thread_rng, Rng};
use indicatif::{ProgressIterator, ParallelProgressIterator};




/*fn record_game<T: Write>(players: [&dyn PlayerController; 2], max_moves: usize, writer: &mut T) -> io::Result<()> {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");

    /*println!("{}", board);
    let move_set = generate_pseudo_legal_moves(&board);
    println!("{} masks", move_set.move_masks().len());
    for m in move_set.moves() {
        println!("{}", m.san());
    }

    board = board.play(move_set.moves().next().unwrap());*/


    let mut index = thread_rng().gen_range(0, 2);
    let colors = if index == 1 {
        [Color::Black, Color::White]
    } else {
        [Color::White, Color::Black]
    };
    assert!(colors[index] == Color::White);
    
    let mut moves = 0;

    write!(writer, "1. ")?;
    loop {
        let color = colors[index];

        if !board.has_king(color) {
            break;
        }

        moves += 1;
        if moves >= max_moves {
            return Ok(());
        }

        let move_set = generate_pseudo_legal_moves(&board);
        if let Some(m) = players[index].play(&move_set) {
            write!(writer, "{} ", m.san())?;
            board = board.play(m);
            index = 1 - index;
        } else {
            break;
        }
    };

    let winner = 1 - index;
    assert!(board.has_king(colors[winner]));

    Ok(())
}


fn main() {
    let mut players = Vec::new();
    players.push(RandomAI::new());
    players.push(RandomAI::new());


    record_game([&players[0], &players[1]], 100, &mut std::io::stdout()).unwrap();
}*/





const GAMES: usize = 50;

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
    let players = vec![
        EloPlayer::new(TreeSearchV2AI(3)),
        EloPlayer::new(AlphaBetaAI(3)),
    ];


    println!("Simulating:");

    let start = Instant::now();
    (0..GAMES).into_par_iter().progress().map(|_| {
        let (a, b) = gen_player_indexes(players.len());
        let (first, second) = (cmp::min(a, b), cmp::max(a, b));
        assert!(a != b);

        let (pa, pb) = players.split_at(second);
        pa[first].play_once(&pb[0]);
    }).count();
    let time = Instant::now() - start;

    println!("\n{} games played in {:?} ({} g/s)", GAMES, time, per_second(GAMES, time).round() as i64);

    let mut total = 0;
    for player in players {
        let history = player.history();
        println!("\n{}", player.name());
        println!("  elo: {}", history.elo_score().round() as i64);
        println!("  time: {:.2}s", history.timer.as_secs_f32());
        println!("  games: (w: {}, d: {}, l: {})", history.victories, history.draws, history.loses);
        total += history.victories + history.loses + history.draws;
    }
    assert!(total == GAMES * 2);
}


