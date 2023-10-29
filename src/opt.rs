// use crate::connectfour::ConnectFour;
use crate::game::Game;
use crate::eval::EvaluationFunction;

pub fn minimax_move(
    game: &mut Game,
    eval_func: &dyn EvaluationFunction,
    search_depth: usize
) -> (i32, usize) {
    let num_players = game.get_num_players();
    let mut alphas = vec![0.0; num_players];
    let (_, best_move) = dfs(game, 0, &mut alphas, eval_func, search_depth);
    let best_move = best_move.expect("No valid move found!");

    (0, best_move)
}

fn dfs(
    game: &mut Game,
    d: usize,
    alphas: &mut Vec<f64>,
    eval_func: &dyn EvaluationFunction,
    search_depth: usize
) -> (Vec<f64>, Option<usize>) {
    let status = game.get_game_status();
    if status != -2 {
        return (game.get_score(), None);
    }
    if d == search_depth {
        return (eval_func.evaluate(game.get_state()), None);
    }

    let player = game.get_to_move();
    let old_alpha = alphas[player];
    let moves = game.get_valid_moves();
    let mut best_move = None;
    let mut best_score: Option<Vec<f64>> = None;
    for &move_col in &moves {
        let saved_state = game.get_state().clone();
        game.transition(move_col);
        let (score, _) = dfs(game, d + 1, alphas, eval_func, search_depth);
        if best_score.is_none() || score[player] > best_score.as_ref().unwrap()[player] {
            best_score = Some(score);
            best_move = Some(move_col);
            if can_prune(best_score.as_ref().unwrap(), &alphas, player) {
                game.set_state(saved_state);
                break;
            }
            alphas[player] = alphas[player].max(best_score.as_ref().unwrap()[player]);
        }
        game.set_state(saved_state);
    }
    alphas[player] = old_alpha;
    (best_score.unwrap(), best_move)
}

fn can_prune(score: &Vec<f64>, alphas: &Vec<f64>, player: usize) -> bool {
    let max_other_player = alphas.iter().enumerate()
        .filter(|&(i, _)| i != player)
        .map(|(_, &alpha)| alpha)
        .fold(f64::NEG_INFINITY, f64::max);

    score[player] > 1.0 - max_other_player
}