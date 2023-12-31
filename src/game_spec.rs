use crate::game::{Game, GameState};
use crate::game_elements::{BoardCell, Player};

pub trait GameSpec {
    fn get_initial_board(&self) -> Vec<Vec<BoardCell>>;
    fn get_initial_to_move(&self) -> Player;
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)>;
    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> Box<GameState>;
    fn is_win(&self, game: &Game, player: Player) -> bool;
    fn is_tie(&self, game: &Game) -> bool;
}
