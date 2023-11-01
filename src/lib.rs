mod eval;
mod opt;
mod game;
mod game_elements;
mod game_traits;
mod push_up_four;
mod user_game;

use wasm_bindgen::prelude::*;
use js_sys::Array;
// use crate::pushupfour::{PushUpFourValidMoves, PushUpFourTransitionFunction, PushUpFourWinCondition, PushUpFourTieCondition};
use crate::user_game::{UserGameInitialState, UserGameValidMoves, UserGameTransitionFunction, UserGameWinCondition, UserGameTieCondition};
use crate::game::Game;
use crate::game_elements::{Player, GameStatus, BoardCell};
use crate::eval::RandomEvaluationFunction;
use crate::opt::minimax_move;

#[wasm_bindgen]
pub struct GameController {
    game: Game,
    eval_function: RandomEvaluationFunction,
}

#[wasm_bindgen]
pub fn create_game_controller(num_players: usize) -> GameController {
    const NUM_ROWS: usize = 6;
    const NUM_COLS: usize = 7;
    let game: Game = Game::new(
        NUM_ROWS,
        NUM_COLS,
        num_players,
        Box::new(UserGameInitialState {}),
        Box::new(UserGameValidMoves {}),
        Box::new(UserGameTransitionFunction {}),
        Box::new(UserGameWinCondition {}),
        Box::new(UserGameTieCondition {}),
    );
    GameController { 
        game,
        eval_function: RandomEvaluationFunction::new(num_players),
    }
}

#[wasm_bindgen]
impl GameController {
    pub fn get_board(&self) -> Array {
        let rust_board = self.game.get_board();
        let js_board = Array::new_with_length(rust_board.len() as u32);
        for (i, row) in rust_board.iter().enumerate() {
            let js_row = Array::new_with_length(row.len() as u32);
            for (j, &cell) in row.iter().enumerate() {
                let value = match cell {
                    BoardCell::Empty => "",
                    BoardCell::Occupied(Player::Player0) => "X",
                    BoardCell::Occupied(Player::Player1) => "O",
                    BoardCell::Occupied(Player::Player2) => "Z",
                    BoardCell::Occupied(Player::Player3) => "W",
                };
                js_row.set(j as u32, JsValue::from_str(value));
            }
            js_board.set(i as u32, js_row.into());
        }
        js_board
    }

    pub fn get_to_move(&self) -> Player {
        self.game.get_to_move()
    }

    pub fn get_move_num(&self) -> usize {
        self.game.get_move_num()
    }

    pub fn get_game_status(&self) -> GameStatus {
        self.game.get_game_status()
    }

    pub fn make_ai_move(&mut self) {
        const SEARCH_DEPTH: usize = 5;
        let (_, (move_row, move_col)) = minimax_move(&mut self.game, &self.eval_function, SEARCH_DEPTH);
        self.game.transition(move_row, move_col);
    }

    pub fn make_human_move(&mut self, move_row: usize, move_col: usize) {
        self.game.transition(move_row, move_col);
    }

    pub fn get_valid_moves(&self) -> Array {
        let valid_moves = Array::new();
        for &(row, col) in self.game.get_valid_moves().iter() {
            let arr = Array::new();
            arr.push(&JsValue::from_f64(row as f64));
            arr.push(&JsValue::from_f64(col as f64));
            valid_moves.push(&arr.into());
        }

        valid_moves
    }
}