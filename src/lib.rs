mod game;
mod pushupfour;
mod eval;
mod opt;
mod gametraits;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::pushupfour::{PushUpFourValidMoves, PushUpFourTransitionFunction, PushUpFourWinCondition, PushUpFourTieCondition};
use crate::game::Game;
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
    const N_IN_A_ROW: usize = 4;
    let game: Game = Game::new(
        NUM_ROWS,
        NUM_COLS,
        num_players,
        N_IN_A_ROW,
        Box::new(PushUpFourValidMoves {}),
        Box::new(PushUpFourTransitionFunction {}),
        Box::new(PushUpFourWinCondition {}),
        Box::new(PushUpFourTieCondition {}),
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
        let js_board = Array::new();
        for row in rust_board.iter() {
            let js_row = Array::new();
            for &cell in row.iter() {
                let value = match cell {
                    -1 => "",
                    0 => "X",
                    1 => "O",
                    2 => "Z",
                    3 => "W",
                    _ => "", // default
                };
                js_row.push(&JsValue::from_str(value));
            }
            js_board.push(&js_row.into());
        }
        js_board
    }

    pub fn get_to_move(&self) -> usize {
        self.game.get_to_move()
    }

    pub fn get_move_num(&self) -> usize {
        self.game.get_move_num()
    }

    pub fn get_game_status(&self) -> i32 {
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