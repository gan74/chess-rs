use crate::board::*;
use crate::piece::*;
use crate::moves::*;
use crate::pos::*;

use rand::prelude::*;

use std::cmp;
use std::io;
use std::str::FromStr;

use rand::{thread_rng, Rng};




pub trait PlayerController : Sync + Send {
    fn name(&self) -> String;
    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>>;
}






pub struct RandomAI();

impl PlayerController for RandomAI {
    fn name(&self) -> String {
        "Random".to_string()
    }

    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>> {
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


pub struct CaptureAI {
    pub search_check: bool,
}

impl PlayerController for CaptureAI {
    fn name(&self) -> String {
        format!("Capture({})", self.search_check)
    }

    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>> {
        let board = moves.parent_board();
        let enemy_king_pos = board.king_pos(board.to_move().opponent());

        if self.search_check {
            let check_move = moves.moves().filter(|mov| {
                let result = board.play(*mov);
                let reach = generate_pseudo_legal_moves_for_color(&result, board.to_move()).all_dst_positions();
                reach.contains(enemy_king_pos)
            }).min_by_key(|mov| board.piece_at(mov.src).kind.score());

            if check_move.is_some() {
                return check_move;
            }
        }

        let captures = board.pieces_for(board.to_move().opponent()).intersect(moves.all_dst_positions());

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



pub struct MonteCarloAI(pub usize);

impl MonteCarloAI {
    fn search<R: RngCore>(board: &Board, rng: &mut R, depth: i64) -> i64 {
        let enemy_king_pos = board.king_pos(board.to_move().opponent());
        let moves = generate_pseudo_legal_moves(&board);

        let score = {
            let cap = 20;
            cap / cmp::min(cap, depth)  
        };

        if moves.is_empty() {
            -score
        } else if moves.all_dst_positions().contains(enemy_king_pos) {
            score
        } else {
            let move_count = moves.moves().count();
            let mov = moves.moves().nth(rng.gen_range(0, move_count)).unwrap();
            -Self::search(&board.play(mov), rng, depth + 1)
        }
    }
}

impl PlayerController for MonteCarloAI {
    fn name(&self) -> String {
        format!("MonteCarlo({})", self.0)
    }

    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>> {
        let board = moves.parent_board();

        moves.moves().max_by_key(|mov| {
            let board = board.play(*mov);
            (0..self.0).into_iter().map(|_| {
                let mut rng = thread_rng();
                -Self::search(&board, &mut rng, 1)
            }).sum::<i64>()
        })
    }
}


pub struct TreeSearchV2AI(pub usize);

impl TreeSearchV2AI {
    const WIN_SCORE: i64 = 10000;

    fn eval(board: Board, depth: usize) -> i64 {
        if depth == 0 {
            return board.pieces_for(board.to_move())
                .set_positions()
                .map(|p| board.piece_at(p).kind.score()).sum::<i64>();
        }

        let moves = generate_pseudo_legal_moves(&board);
        let enemy_king_pos = board.king_pos(board.to_move().opponent());

        if moves.is_empty() {
            -Self::WIN_SCORE
        } else if moves.all_dst_positions().contains(enemy_king_pos) {
            Self::WIN_SCORE
        } else { 
            let score = moves.moves().map(|mov| Self::eval(board.play(mov), depth - 1)).min().unwrap();
            (-score * 9) / 10 
        }

    }
}

impl PlayerController for TreeSearchV2AI {
    fn name(&self) -> String {
        format!("TreeSearchV2({})", self.0)
    }

    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>> {
        moves.moves().max_by_key(|mov| {
            let board = mov.parent_board().play(*mov);
            -Self::eval(board, self.0)
        })
    }
}



pub struct AlphaBetaAI(pub usize);

impl AlphaBetaAI {
    const WIN_SCORE: i64 = 10000;

    fn eval(board: Board, depth: usize, mut alpha: i64, mut beta: i64, is_max_player: bool) -> i64 {
        let sign = if is_max_player { 1 } else { -1 };
        if depth == 0 {
            return board.pieces_for(board.to_move())
                .set_positions()
                .map(|p| board.piece_at(p).kind.score()).sum::<i64>() * sign;
        }

        let moves = generate_pseudo_legal_moves(&board);
        if moves.is_empty() {
            return -Self::WIN_SCORE * sign;
        }


        let enemy_king_pos = board.king_pos(board.to_move().opponent());
        if moves.all_dst_positions().contains(enemy_king_pos) {
            return Self::WIN_SCORE * sign;
        }


        if is_max_player {
            let mut best = i64::MIN;
            for mov in moves.moves() {
                best = cmp::max(best, Self::eval(board.play(mov), depth - 1, alpha, beta, false));
                alpha = cmp::max(alpha, best);
                if best >= beta {
                    break
                }
            }
            best
        } else {
            let mut best = i64::MAX;
            for mov in moves.moves() {
                best = cmp::min(best, Self::eval(board.play(mov), depth - 1, alpha, beta, true));
                beta = cmp::min(beta, best);
                if best <= alpha {
                    break
                }
            }
            best
        }
    }
}

impl PlayerController for AlphaBetaAI {
    fn name(&self) -> String {
        format!("AlphaBeta({})", self.0)
    }

    fn play<'a>(&self, moves: &'a MoveSet) -> Option<Move<'a>> {
        let mut best_score = i64::MIN;
        let mut best_move: Option<Move> = None;

        for mov in moves.moves() {
            let board = mov.parent_board().play(mov);
            let v = Self::eval(board, self.0, best_score, i64::MAX, false);
            if v > best_score {
                best_score = v;
                best_move = Some(mov);
            }
        }
        best_move
    }
}