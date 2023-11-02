use crate::game::{Game, GameState};
use crate::game_elements::{BoardCell, Player};

pub trait GameSpec {
    fn get_initial_board(&self, num_rows: usize, num_cols: usize) -> Vec<Vec<BoardCell>>;
    fn get_initial_to_move(&self) -> Player;
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)>;
    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> GameState;
    fn is_win(&self, game: &Game, player: Player) -> bool;
    fn is_tie(&self, game: &Game) -> bool;
}
