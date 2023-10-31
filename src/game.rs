// use std::mem;
use crate::gametraits::{MoveValidator, GameTransition, WinCondition, TieCondition};

// TODO: why do we need Clone here and not just copy (if all of the fields are copyable?)
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

pub struct Game {
    state: GameState,
    num_rows: usize,
    num_cols: usize,
    num_players: usize,
    n_in_a_row: usize,
    move_validator: Box<dyn MoveValidator>,
    game_transition: Box<dyn GameTransition>,
    win_condition: Box<dyn WinCondition>,
    tie_condition: Box<dyn TieCondition>,
}

impl Game {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        num_players: usize,
        n_in_a_row: usize,
        move_validator: Box<dyn MoveValidator>,
        game_transition: Box<dyn GameTransition>,
        win_condition: Box<dyn WinCondition>,
        tie_condition: Box<dyn TieCondition>,
    ) -> Game {
        Game {
            state: GameState::new(0, 0, vec![vec![-1; num_cols]; num_rows]),
            num_rows,
            num_cols,
            num_players,
            n_in_a_row,
            move_validator,
            game_transition,
            win_condition,
            tie_condition,
        }
    }

    pub fn get_game_status(&self) -> i32 {
        if self.is_tie() {
            return -1;
        }
        for player in 0..self.num_players {
            if self.is_win(player) {
                return player as i32;
            }
        }
        -2
    }

    pub fn get_score(&self) -> Vec<f64> {
        let res = self.get_game_status();
        if res < 0 {
            return vec![1.0 / self.num_players as f64; self.num_players];
        } else {
            let mut score = vec![0.0; self.num_players];
            score[res as usize] = 1.0;
            score
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn get_num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn get_num_players(&self) -> usize {
        self.num_players
    }

    pub fn get_n_in_a_row(&self) -> usize {
        self.n_in_a_row
    }

    pub fn get_board(&self) -> &Vec<Vec<i32>> {
        self.state.get_board()
    }

    pub fn get_to_move(&self) -> usize {
        self.state.get_to_move()
    }

    pub fn get_move_num(&self) -> usize {
        self.state.get_move_num()
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    // Below: functions that call the dependency injected functions
    pub fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        self.move_validator.get_valid_moves(self)
    }

    pub fn transition(&mut self, move_row: usize, move_col: usize) {
        self.state = self.game_transition.transition(self, move_row, move_col);
    }

    pub fn is_win(&self, player: usize) -> bool {
        self.win_condition.is_win(self, player)
    }

    pub fn is_tie(&self) -> bool {
        self.tie_condition.is_tie(self)
    }
}