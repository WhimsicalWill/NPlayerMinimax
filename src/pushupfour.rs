use crate::gametraits::{ValidMoves, TransitionFunction, WinCondition, TieCondition};
use crate::game::{Game, GameState};

pub struct PushUpFourValidMoves;
impl ValidMoves for PushUpFourValidMoves {
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let bottom_row = game.get_num_rows() - 1;
        (0..game.get_num_cols())
            .filter_map(|col| {
                if game.get_state().get_board()[0][col] == -1 {
                    Some((bottom_row, col))  // (row, col) with origin at the top left
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct PushUpFourTransitionFunction;
impl TransitionFunction for PushUpFourTransitionFunction {
    fn transition(&self, game: &Game, _move_row: usize, move_col: usize) -> GameState {
        let mut board_copy = game.get_state().get_board().clone();
        
        // Push the new chip up the bottom, shifting other chips up
        for row in 0..game.get_num_rows() - 1 {
            board_copy[row][move_col] = board_copy[row + 1][move_col];
        }
        board_copy[game.get_num_rows() - 1][move_col] = game.get_state().get_to_move() as i32;
        
        GameState::new(
            (game.get_state().get_to_move() + 1) % game.get_num_players(), 
            game.get_state().get_move_num() + 1, 
            board_copy
        )
    }
}

pub struct PushUpFourWinCondition;
impl WinCondition for PushUpFourWinCondition {
    fn is_win(&self, game: &Game, player: usize) -> bool {
        is_win(game, player)
    }
}

pub struct PushUpFourTieCondition;
impl TieCondition for PushUpFourTieCondition {
    fn is_tie(&self, game: &Game) -> bool {
        game.get_state().get_move_num() == game.get_num_rows() * game.get_num_cols() || is_tie_after_transition(game)
    }
}

pub fn is_sequence_win(sequence: &[i32], player: usize, n_in_a_row: usize) -> bool {
    sequence.iter().filter(|&&x| x == player as i32).count() >= n_in_a_row
}

pub fn is_row_win(game: &Game, player: usize) -> bool {
    let num_cols = game.get_num_cols();
    let n_in_a_row = game.get_n_in_a_row();

    for row in game.get_state().get_board().iter() {
        for i in 0..num_cols - n_in_a_row + 1 {
            if is_sequence_win(&row[i..i+n_in_a_row], player, n_in_a_row) {
                return true;
            }
        }
    }
    false
}

pub fn is_col_win(game: &Game, player: usize) -> bool {
    let num_cols = game.get_num_cols();
    let num_rows = game.get_num_rows();
    let n_in_a_row = game.get_n_in_a_row();

    for col in 0..num_cols {
        let col_elems: Vec<i32> = game.get_state().get_board().iter().map(|row| row[col]).collect();
        for i in 0..num_rows - n_in_a_row + 1 {
            if is_sequence_win(&col_elems[i..i + n_in_a_row], player, n_in_a_row) {
                return true;
            }
        }
    }
    false
}

pub fn is_diag_win(game: &Game, player: usize) -> bool {
    let num_rows = game.get_num_rows();
    let num_cols = game.get_num_cols();
    let n_in_a_row = game.get_n_in_a_row();

    // Check main diagonals
    for i in 0..num_rows - n_in_a_row + 1 {
        for j in 0..num_cols - n_in_a_row + 1 {
            let diagonal: Vec<i32> = (0..n_in_a_row).map(|k| game.get_state().get_board()[i + k][j + k]).collect();
            if is_sequence_win(&diagonal, player, n_in_a_row) {
                return true;
            }
        }
    }

    // Check counter-diagonals
    for i in 0..num_rows - n_in_a_row + 1 {
        for j in n_in_a_row - 1..num_cols {
            let diagonal: Vec<i32> = (0..n_in_a_row).map(|k| game.get_state().get_board()[i + k][j - k]).collect();
            if is_sequence_win(&diagonal, player, n_in_a_row) {
                return true;
            }
        }
    }
    false
}

fn is_tie_after_transition(game: &Game) -> bool {
    let num_players = game.get_num_players();
    let winners: Vec<bool> = (0..num_players).map(|player| is_win(game, player)).collect();
    winners.iter().filter(|&&x| x).count() > 1
}

// Note: You might need a similar `is_win` function that works with game: &Game
pub fn is_win(game: &Game, player: usize) -> bool {
    is_row_win(game, player) || is_col_win(game, player) || is_diag_win(game, player)
}