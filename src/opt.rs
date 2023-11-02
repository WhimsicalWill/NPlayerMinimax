// use crate::connectfour::ConnectFour;
use crate::eval::EvaluationFunction;
use crate::game::Game;
use crate::game_elements::GameStatus;

pub fn minimax_move(
    game: &mut Game,
    eval_func: &dyn EvaluationFunction,
    search_depth: usize,
) -> (f64, Option<(usize, usize)>) {
    // Adjusted return type if needed
    let num_players = game.get_num_players();
    let mut alphas = vec![f64::NEG_INFINITY; num_players];
    let (best_score, best_move) = dfs(game, 0, &mut alphas, eval_func, search_depth);

    (
        best_score[game.get_state().get_to_move().to_usize()],
        best_move,
    )
}

fn dfs(
    game: &mut Game,
    d: usize,
    alphas: &mut Vec<f64>,
    eval_func: &dyn EvaluationFunction,
    search_depth: usize,
) -> (Vec<f64>, Option<(usize, usize)>) {
    let status = game.get_game_status();
    if status != GameStatus::Ongoing {
        return (game.get_score(), None);
    }
    if d == search_depth {
        return (eval_func.evaluate(game.get_state()), None);
    }

    let player_idx = game.get_to_move().to_usize();
    let old_alpha = alphas[player_idx];
    let moves = game.get_valid_moves();
    let mut best_move = None;
    let mut best_score: Option<Vec<f64>> = None;
    for &(move_row, move_col) in &moves {
        let saved_state = game.get_state().clone();
        game.transition(move_row, move_col); // needs change
        let (score, _) = dfs(game, d + 1, alphas, eval_func, search_depth);
        if best_score.is_none() || score[player_idx] > best_score.as_ref().unwrap()[player_idx] {
            best_score = Some(score);
            best_move = Some((move_row, move_col)); // needs change
            if can_prune(best_score.as_ref().unwrap(), &alphas, player_idx) {
                game.set_state(saved_state);
                break;
            }
            alphas[player_idx] = alphas[player_idx].max(best_score.as_ref().unwrap()[player_idx]);
        }
        game.set_state(saved_state);
    }
    alphas[player_idx] = old_alpha;
    // return all 0s if there are no available moves yet the game is ongoing (shouldn't happen)
    (
        best_score.unwrap_or_else(|| vec![0.0; game.get_num_players()]),
        best_move,
    )
}

fn can_prune(score: &Vec<f64>, alphas: &Vec<f64>, player_idx: usize) -> bool {
    let max_other_player = alphas
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != player_idx)
        .map(|(_, &alpha)| alpha)
        .fold(f64::NEG_INFINITY, f64::max);

    score[player_idx] > 1.0 - max_other_player
}
