use crate::gametraits::{ValidMoves, TransitionFunction, WinCondition, TieCondition};
use crate::game::{Game, GameState};
use std::collections::HashSet;

pub struct GoValidMoves;
impl ValidMoves for GoValidMoves {
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let board = game.get_state().get_board();
        let mut valid_moves = Vec::new();

        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == -1 { // Assuming -1 represents an empty cell
                    valid_moves.push((row_idx, col_idx));
                }
            }
        }
        valid_moves
    }
}

pub struct GoTransitionFunction;
impl TransitionFunction for GoTransitionFunction {
    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> GameState {
        let mut board_copy = game.get_state().get_board().clone();
        let current_player = game.get_state().get_to_move();
        board_copy[move_row][move_col] = current_player as i32;

        // Remove captured stones
        let opponent = if current_player == 0 { 1 } else { 0 };
        remove_captured_stones(&mut board_copy, opponent);

        GameState::new(
            (current_player + 1) % game.get_num_players(),
            game.get_state().get_move_num() + 1,
            board_copy
        )
    }
}

pub struct GoWinCondition;
impl WinCondition for GoWinCondition {
    fn is_win(&self, _game: &Game, _player: usize) -> bool {
        // Implement scoring logic to determine winner
        // This could be complex, involving counting territory and captured stones
        false
    }
}

pub struct GoTieCondition;
impl TieCondition for GoTieCondition {
    fn is_tie(&self, _game: &Game) -> bool {
        // Implement scoring logic to determine winner
        // This could be complex, involving counting territory and captured stones
        false
    }
}

fn remove_captured_stones(board: &mut Vec<Vec<i32>>, player: i32) {
    let mut to_remove = HashSet::new();
    let board_size = board.len();

    for row in 0..board_size {
        for col in 0..board[row].len() {
            if board[row][col] == player && is_captured(board, row, col, &mut HashSet::new()) {
                to_remove.insert((row, col));
            }
        }
    }

    for &(row, col) in &to_remove {
        board[row][col] = -1; // Assuming -1 is empty
    }
}

fn is_captured(board: &Vec<Vec<i32>>, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>) -> bool {
    if visited.contains(&(row, col)) {
        return true;
    }

    visited.insert((row, col));

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Adjacent cells
    let stone = board[row][col];

    for &(dx, dy) in &directions {
        let (new_row, new_col) = (row as i32 + dx, col as i32 + dy);

        if new_row >= 0 && new_col >= 0 && (new_row as usize) < board.len() && (new_col as usize) < board[row].len() {
            if board[new_row as usize][new_col as usize] == -1 {
                // Liberty found
                return false;
            } else if board[new_row as usize][new_col as usize] == stone {
                // Continue searching
                if !is_captured(board, new_row as usize, new_col as usize, visited) {
                    return false;
                }
            }
        }
    }

    true
}