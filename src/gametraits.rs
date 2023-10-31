use crate::game::{Game, GameState};

pub trait ValidMoves {
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)>;
}

pub trait TransitionFunction {
    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> GameState;
}

pub trait WinCondition {
    fn is_win(&self, game: &Game, player: usize) -> bool;
}

pub trait TieCondition {
    fn is_tie(&self, game: &Game) -> bool;
}