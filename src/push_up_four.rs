// Game Name: PushUpFour
// InitialState: Empty board; Player 0 moves first
// ValidMoves: Players can place chips on the bottom row of columns that aren't full.
// TransitionFunction: Chips are placed at the bottom of the column, pushing other chips up.
// WinCondition: There are n consecutive chips in a row, column, or diagonal.
// TieCondition: Either more than one player wins simultaneously or the board fills up.

use crate::game::{Game, GameState};
use crate::game_elements::{BoardCell, Player};
use crate::game_spec::GameSpec;

const N_IN_A_ROW: usize = 4;

pub struct PushUpFourSpec;
impl GameSpec for PushUpFourSpec {
    fn get_initial_board(&self, rows: usize, cols: usize) -> Vec<Vec<BoardCell>> {
        vec![vec![None; cols]; rows]
    }

    fn get_initial_to_move(&self) -> Player {
        Player::Player0
    }

    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let bottom_row = game.get_num_rows() - 1;
        (0..game.get_num_cols())
            .filter_map(|col| {
                match game.get_board()[0][col] {
                    None => Some((bottom_row, col)), // (row, col) with origin at the top left
                    _ => None,
                }
            })
            .collect()
    }

    fn transition(&self, game: &Game, _move_row: usize, move_col: usize) -> GameState {
        let mut board_copy = game.get_board().clone();

        // Push the new chip up the bottom, shifting other chips up
        for row in 0..game.get_num_rows() - 1 {
            board_copy[row][move_col] = board_copy[row + 1][move_col];
        }
        board_copy[game.get_num_rows() - 1][move_col] = Some(game.get_to_move());

        // Update the player to move, the move number, and the state of the baord
        GameState::new(game.get_next_player(), game.get_move_num() + 1, board_copy)
    }

    fn is_win(&self, game: &Game, player: Player) -> bool {
        self.is_row_win(game, player)
            || self.is_col_win(game, player)
            || self.is_diag_win(game, player)
    }

    fn is_tie(&self, game: &Game) -> bool {
        game.get_move_num() == game.get_num_rows() * game.get_num_cols()
            || self.is_tie_after_transition(game)
    }
}

impl PushUpFourSpec {
    fn is_sequence_win(&self, sequence: &[BoardCell], player: Player) -> bool {
        sequence
            .iter()
            .filter(|&cell| *cell == Some(player))
            .count()
            >= N_IN_A_ROW
    }

    fn is_row_win(&self, game: &Game, player: Player) -> bool {
        let num_cols = game.get_num_cols();
        for row in game.get_state().get_board().iter() {
            for i in 0..num_cols - N_IN_A_ROW + 1 {
                if self.is_sequence_win(&row[i..i + N_IN_A_ROW], player) {
                    return true;
                }
            }
        }
        false
    }

    fn is_col_win(&self, game: &Game, player: Player) -> bool {
        let num_cols = game.get_num_cols();
        let num_rows = game.get_num_rows();
        for col in 0..num_cols {
            let col_elems: Vec<BoardCell> = game
                .get_state()
                .get_board()
                .iter()
                .map(|row| row[col])
                .collect();
            for i in 0..num_rows - N_IN_A_ROW + 1 {
                if self.is_sequence_win(&col_elems[i..i + N_IN_A_ROW], player) {
                    return true;
                }
            }
        }
        false
    }

    fn is_diag_win(&self, game: &Game, player: Player) -> bool {
        let num_rows = game.get_num_rows();
        let num_cols = game.get_num_cols();

        // Check main diagonals
        for i in 0..num_rows - N_IN_A_ROW + 1 {
            for j in 0..num_cols - N_IN_A_ROW + 1 {
                let diagonal: Vec<BoardCell> = (0..N_IN_A_ROW)
                    .map(|k| game.get_state().get_board()[i + k][j + k])
                    .collect();
                if self.is_sequence_win(&diagonal, player) {
                    return true;
                }
            }
        }

        // Check counter-diagonals
        for i in 0..num_rows - N_IN_A_ROW + 1 {
            for j in N_IN_A_ROW - 1..num_cols {
                let diagonal: Vec<BoardCell> = (0..N_IN_A_ROW)
                    .map(|k| game.get_state().get_board()[i + k][j - k])
                    .collect();
                if self.is_sequence_win(&diagonal, player) {
                    return true;
                }
            }
        }
        false
    }

    fn is_tie_after_transition(&self, game: &Game) -> bool {
        let num_players = game.get_num_players();
        let winners: Vec<bool> = (0..num_players)
            .map(|player| self.is_win(game, Player::from(player)))
            .collect();
        winners.iter().filter(|&&x| x).count() > 1
    }
}
