use crate::player::*;
use crate::board::*;
use crate::piece::*;
use crate::moves::*;

use rand::{thread_rng, Rng};

use std::sync::Mutex;
use std::time::{Duration, Instant};


const ELO_STARTING_SCORE : i64 = 1200;
const ELO_K : f64 = 1.0;
const MAX_MOVES : usize = 100;


#[derive(Debug, Clone)]
struct Elo {
    score: f64
}

#[derive(Debug, Clone)]
pub struct History {
    elo: Elo,

    pub timer: Duration,
    pub victories: usize,
    pub loses: usize,
    pub draws: usize,
}


pub struct EloPlayer {
    controller: Box<dyn PlayerController>,
    pub history: Mutex<History>,
}





impl Elo {
    pub fn new() -> Elo {
        Elo {
            score: ELO_STARTING_SCORE as f64,
        }
    }

    pub fn win(&mut self, other: &mut Elo, k: f64) {
        let diff = k * (1.0 - self.win_probability(other));
        self.score += diff;
        other.score -= diff;
    }

    pub fn lose(&mut self, other: &mut Elo, k: f64) {
        self.win(other, -k);
    }


    fn win_probability(&self, other: &Elo) -> f64 {
        let p0 = 1.0 / (1.0 + 10.0_f64.powf((other.score - self.score) / 400.0));
        debug_assert!(p0 >= 0.0);
        debug_assert!(p0 <= 1.0);
        p0
    }
}

impl History {
    pub fn elo_score(&self) -> f64 {
        self.elo.score
    }

    pub fn win(&mut self, other: &mut History, k: f64) {
        self.elo.win(&mut other.elo, k);
        self.victories += 1;
        other.loses += 1;
    }

    pub fn draw(&mut self, other: &mut History) {
        self.draws += 1;
        other.draws += 1;
    }
}

impl EloPlayer {
    pub fn new<T: PlayerController + 'static>(player: T) -> EloPlayer {
        EloPlayer {
            controller: Box::new(player),
            history: Mutex::new(
                History {
                    elo: Elo::new(),
                    timer: Duration::ZERO,
                    victories: 0,
                    loses: 0,
                    draws: 0,
                }
            ),
        }
    }

    pub fn name(&self) -> String {
        self.controller.name()
    }

    pub fn history(&self) -> History {
        self.history.lock().unwrap().clone()
    }

    pub fn play_once(&self, other: &EloPlayer) {
        let players = [&(*self.controller), &(*other.controller)];
        let (result, durations) = play_once(players);
        {
            let mut hist_s = self.history.lock().unwrap();
            let mut hist_o = other.history.lock().unwrap();
            hist_s.timer += durations[0];
            hist_o.timer += durations[1];
            match result {
                Some(0) => hist_s.win(&mut hist_o, ELO_K),
                Some(1) => hist_o.win(&mut hist_s, ELO_K),
                _ => hist_s.draw(&mut hist_o),
            }
        }
    }
}



fn play_once(players: [&dyn PlayerController; 2]) -> (Option<usize>, [Duration; 2]) {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");
    let mut player_index = thread_rng().gen_range(0, 2);
    let mut durations = [Duration::ZERO; 2];

    let mut moves_left = MAX_MOVES;
    loop {
        if moves_left == 0 {
            return (None, durations);
        }
        moves_left -= 1;

        if !board.has_king(board.to_move()) {
            break;
        }

        let enemy_king_pos = board.king_pos(board.to_move().opponent());
        let move_set = generate_pseudo_legal_moves(&board);
        if move_set.all_dst_positions().contains(enemy_king_pos) {
            player_index = 1 - player_index;
            break;
        }

        if move_set.is_empty() {
            break;
        }

        let start = Instant::now();
        let chosen = players[player_index].play(&move_set);
        durations[player_index] += Instant::now() - start;

        if let Some(mov) = chosen {
            board = board.play(mov);
            player_index = 1 - player_index;
        } else {
            break;
        }
    };

    let winner = 1 - player_index;
    (Some(winner), durations)
}
