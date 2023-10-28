#[derive(Clone)]
pub struct GameState {
    to_move: usize,
    move_num: usize,
    board: Vec<Vec<i32>>,
}

impl GameState {
    pub fn new(to_move: usize, move_num: usize, board: Vec<Vec<i32>>) -> Self {
        GameState {
            to_move,
            move_num,
            board,
        }
    }

    pub fn get_to_move(&self) -> usize {
        self.to_move
    }

    pub fn get_move_num(&self) -> usize {
        self.move_num
    }

    pub fn get_board(&self) -> &Vec<Vec<i32>> {
        &self.board
    }
}

pub trait Game {
    fn get_score(&self) -> Vec<f64>;
    fn get_game_status(&self) -> i32;
    fn get_state(&self) -> &GameState;
    fn set_state(&mut self, state: GameState);
    fn get_valid_moves(&self) -> Vec<usize>;
    fn transition(&mut self, move_col: usize);
    fn is_win(&self, player: usize) -> bool;
    fn is_tie(&self) -> bool;
    fn get_user_move(&self) -> usize;
    fn print(&self);
}