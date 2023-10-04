use crate::player::*;
use crate::board::*;
use crate::piece::*;
use crate::moves::*;

use rand::{thread_rng, Rng};



const ELO_STARTING_SCORE : i64 = 1200;
const ELO_K : f64 = 1.0;
const MAX_MOVES : usize = 100;


#[derive(Debug, Clone, Copy)]
pub struct Elo {
    score: f64
}


pub struct EloPlayer {
    controller: Box<dyn PlayerController>,
    elo: Elo,

    pub victories: usize,
    pub loses: usize,
    pub draws: usize,
}





impl Elo {
    pub fn new() -> Elo {
        Elo {
            score: ELO_STARTING_SCORE as f64,
        }
    }

    pub fn win(&mut self, other: &mut Elo, k: f64) {
        let diff = k * (1.0 - self.win_probability(*other));
        self.score += diff;
        other.score -= diff;
    }

    pub fn lose(&mut self, other: &mut Elo, k: f64) {
        self.win(other, -k);
    }


    fn win_probability(&self, other: Elo) -> f64 {
        let p0 = 1.0 / (1.0 + 10.0_f64.powf((other.score - self.score) / 400.0));
        debug_assert!(p0 >= 0.0);
        debug_assert!(p0 <= 1.0);
        p0
    }
}


impl EloPlayer {
    pub fn new<T: 'static + PlayerController>(player: T) -> EloPlayer {
        EloPlayer {
            controller: Box::new(player),
            elo: Elo::new(),

            victories: 0,
            loses: 0,
            draws: 0,
        }
    }

    pub fn name(&self) -> String {
        self.controller.name()
    }

    pub fn elo_score(&self) -> f64 {
        self.elo.score
    }

    pub fn win(&mut self, other: &mut EloPlayer, k: f64) {
        self.elo.win(&mut other.elo, k);
        self.victories += 1;
        other.loses += 1;
    }

    pub fn draw(&mut self, other: &mut EloPlayer) {
        self.draws += 1;
        other.draws += 1;
    }

    pub fn play_once(&mut self, other: &mut EloPlayer) -> usize {
        let players = [&(*self.controller), &(*other.controller)];
        let (result, moves) = play_once(players, MAX_MOVES);
        match result {
            Some(0) => self.win(other, ELO_K),
            Some(1) => other.win(self, ELO_K),
            _ => self.draw(other)
        }
        moves
    }
}



fn play_once(players: [&dyn PlayerController; 2], max_moves: usize) -> (Option<usize>, usize) {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");

    let mut index = thread_rng().gen_range(0, 2);
    let colors = if index == 1 {
        [Color::Black, Color::White]
    } else {
        [Color::White, Color::Black]
    };
    assert!(colors[index] == Color::White);
    
    let mut moves = 0;
    loop {
        let color = colors[index];

        if !board.has_king(color) {
            break;
        }

        moves += 1;
        if moves >= max_moves {
            return (None, max_moves);
        }

        let move_set = generate_pseudo_legal_moves(&board);
        if let Some(m) = players[index].play(&move_set) {
            board = board.with_move(m);
            index = 1 - index;
        } else {
            break;
        }
    };

    let winner = 1 - index;

    debug_assert!(board.has_king(colors[winner]));
    (Some(winner), moves)
}
