use crate::board::*;
use crate::piece::*;
use crate::moves::*;
use crate::pos::*;

use std::cmp;
use std::io;
use std::str::FromStr;

use rand::{thread_rng, Rng};




pub trait PlayerController {
    fn name(&self) -> String;
    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>>;
}






pub struct RandomAI();
pub struct CaptureAI();


impl PlayerController for RandomAI {
    fn name(&self) -> String {
        "Random".to_string()
    }

    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>> {
        let board = moves.parent_board();
        let enemy_king_pos = board.king_pos(board.to_move().opponent());

        if let Some(m) = moves.moves_ending_in(enemy_king_pos).next() {
            return Some(m);
        }

        match moves.moves().count() {
            0 => None,
            l => {
                let mut rng = thread_rng();
                let index = rng.gen_range(0, l);
                moves.moves().nth(index)
            },
        }
    }
}


impl PlayerController for CaptureAI {
    fn name(&self) -> String {
        "Capture".to_string()
    }

    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>> {
        let board = moves.parent_board();
        let captures = moves.opponent_pieces().intersect(moves.all_dst_positions());

        if captures.is_empty() {
            match moves.moves().count() {
                0 => None,
                l => {
                    let mut rng = thread_rng();
                    let index = rng.gen_range(0, l);
                    moves.moves().nth(index)
                },
            }
        } else {
            let best_cap = captures.set_positions().max_by_key(|pos| board.piece_at(*pos).kind.score());
            best_cap.map(|pos| moves.moves_ending_in(pos).min_by_key(|mov| board.piece_at(mov.src).kind.score())).flatten()
        }
    }
}



