use crate::game::{Game, GameState};

pub trait AvailableMoves {
    // The ai function doesn't include redundant moves; this makes search faster
    fn get_valid_ai_moves(&self, game: &Game) -> Vec<(usize, usize)>;
    fn get_valid_human_moves(&self, game: &Game) -> Vec<(usize, usize)>;
}

pub trait GameTransition {
    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> GameState;
}

pub trait WinCondition {
    fn is_win(&self, game: &Game, player: usize) -> bool;
}

pub trait TieCondition {
    fn is_tie(&self, game: &Game) -> bool;
}