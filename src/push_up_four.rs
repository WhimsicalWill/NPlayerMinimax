// Name: PushUpFour
// InitialState: Empty board
// ValidMoves: Players can place chips on the bottom row of columns that aren't full.
// TransitionFunction: Chips are placed at the bottom of the column, pushing other chips up.
// WinCondition: There are n consecutive chips in a row, column, or diagonal.
// TieCondition: Either more than one player wins simultaneously or the board fills up.

use crate::game::{Game, GameState};
use crate::game_elements::{Player, BoardCell};
use crate::game_traits::{InitialState, ValidMoves, TransitionFunction, WinCondition, TieCondition};

const N_IN_A_ROW: usize = 4;

pub struct PushUpFourInitialState;
impl InitialState for PushUpFourInitialState {
    fn get_board(&self, rows: usize, cols: usize) -> Vec<Vec<BoardCell>> {
        vec![vec![BoardCell::Empty; cols]; rows]
    }

    fn get_to_move(&self) -> Player {
        Player::Player0
    }

    fn get_move_num(&self) -> usize {
        0
    }
}

pub struct PushUpFourValidMoves;
impl ValidMoves for PushUpFourValidMoves {
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let bottom_row = game.get_num_rows() - 1;
        (0..game.get_num_cols())
            .filter_map(|col| {
                if matches!(game.get_state().get_board()[0][col], BoardCell::Empty) {
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
        board_copy[game.get_num_rows() - 1][move_col] = BoardCell::Occupied(game.get_state().get_to_move());
        
        // Get next player
        let num_players = game.get_num_players();
        let next_player_num = (Player::to_usize(&game.get_state().get_to_move()) + 1) % num_players;
        let next_player = Player::from(next_player_num);

        GameState::new(
            next_player,
            game.get_state().get_move_num() + 1,
            board_copy
        )
    }
}

pub struct PushUpFourWinCondition;
impl WinCondition for PushUpFourWinCondition {
    fn is_win(&self, game: &Game, player: Player) -> bool {
        is_win(game, player)
    }
}

pub struct PushUpFourTieCondition;
impl TieCondition for PushUpFourTieCondition {
    fn is_tie(&self, game: &Game) -> bool {
        game.get_state().get_move_num() == game.get_num_rows() * game.get_num_cols() || is_tie_after_transition(game)
    }
}

pub fn is_sequence_win(sequence: &[BoardCell], player: Player) -> bool {
    sequence.iter().filter(|&&cell| cell == BoardCell::Occupied(player)).count() >= N_IN_A_ROW
}

pub fn is_row_win(game: &Game, player: Player) -> bool {
    let num_cols = game.get_num_cols();
    for row in game.get_state().get_board().iter() {
        for i in 0..num_cols - N_IN_A_ROW + 1 {
            if is_sequence_win(&row[i..i+N_IN_A_ROW], player) {
                return true;
            }
        }
    }
    false
}

pub fn is_col_win(game: &Game, player: Player) -> bool {
    let num_cols = game.get_num_cols();
    let num_rows = game.get_num_rows();
    for col in 0..num_cols {
        let col_elems: Vec<BoardCell> = game.get_state().get_board().iter().map(|row| row[col]).collect();
        for i in 0..num_rows - N_IN_A_ROW + 1 {
            if is_sequence_win(&col_elems[i..i + N_IN_A_ROW], player) {
                return true;
            }
        }
    }
    false
}

pub fn is_diag_win(game: &Game, player: Player) -> bool {
    let num_rows = game.get_num_rows();
    let num_cols = game.get_num_cols();

    // Check main diagonals
    for i in 0..num_rows - N_IN_A_ROW + 1 {
        for j in 0..num_cols - N_IN_A_ROW + 1 {
            let diagonal: Vec<BoardCell> = (0..N_IN_A_ROW).map(|k| game.get_state().get_board()[i + k][j + k]).collect();
            if is_sequence_win(&diagonal, player) {
                return true;
            }
        }
    }

    // Check counter-diagonals
    for i in 0..num_rows - N_IN_A_ROW + 1 {
        for j in N_IN_A_ROW - 1..num_cols {
            let diagonal: Vec<BoardCell> = (0..N_IN_A_ROW).map(|k| game.get_state().get_board()[i + k][j - k]).collect();
            if is_sequence_win(&diagonal, player) {
                return true;
            }
        }
    }
    false
}

fn is_tie_after_transition(game: &Game) -> bool {
    let num_players = game.get_num_players();
    let winners: Vec<bool> = (0..num_players).
        map(|player| is_win(game, Player::from(player))).collect();
    winners.iter().filter(|&&x| x).count() > 1
}

pub fn is_win(game: &Game, player: Player) -> bool {
    is_row_win(game, player) || is_col_win(game, player) || is_diag_win(game, player)
}