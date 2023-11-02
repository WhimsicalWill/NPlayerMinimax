use crate::game::{Game, GameState};
use crate::game_elements::{BoardCell, Player};

pub trait InitialState {
    fn get_board(&self, rows: usize, cols: usize) -> Vec<Vec<BoardCell>>;
    fn get_to_move(&self) -> Player;
    fn get_move_num(&self) -> usize;
}

pub trait ValidMoves {
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)>;
}

pub trait TransitionFunction {
    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> GameState;
}

pub trait WinCondition {
    fn is_win(&self, game: &Game, player: Player) -> bool;
}

pub trait TieCondition {
    fn is_tie(&self, game: &Game) -> bool;
}
