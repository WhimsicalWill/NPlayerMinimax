use crate::connectfour::ConnectFour;
use crate::game::Game;
use crate::eval::EvaluationFunction;
use rand::prelude::SliceRandom;

pub fn minimax_move(game: &ConnectFour, eval_func: &dyn EvaluationFunction) -> (i32, usize) {
    let valid_moves = game.get_valid_moves();
    let random_move = valid_moves.choose(&mut rand::thread_rng()).expect("No valid moves available!");

    (0, *random_move)
}