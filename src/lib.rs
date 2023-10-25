mod game;
mod tictactoe;
// mod connectfour;
mod pushupfour;
mod eval;
mod opt;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::pushupfour::PushUpFour;
use crate::game::Game;
use crate::eval::RandomEvaluationFunction;
use crate::opt::minimax_move;

pub struct PushUpFourController {
    game: PushUpFour,
    eval_function: RandomEvaluationFunction,
    search_depth: usize,
}

impl PushUpFourController {
    pub fn new(n_rows: usize, n_cols: usize, num_players: usize, n_in_a_row: usize) -> PushUpFourController {
        PushUpFourController {
            game: PushUpFour::new(n_rows, n_cols, num_players, n_in_a_row),
            eval_function: RandomEvaluationFunction::new(num_players),
            search_depth: 5,
        }
    }

    // Make a move and get the AI's move (for simplicity)
    pub fn make_move(&mut self, col: usize) {
        self.game.transition(col);

        // AI's move logic, using minimax_move or any other method you have

        let (_, ai_move) = minimax_move(&mut self.game, &self.eval_function, self.search_depth);
        self.game.transition(ai_move);
    }
}

// Static mutable reference to store the game state
static mut GAME_CONTROLLER: Option<PushUpFourController> = None;

#[wasm_bindgen]
pub fn initialize_game(n_rows: usize, n_cols: usize, num_players: usize, n_in_a_row: usize) {
    unsafe {
        GAME_CONTROLLER = Some(PushUpFourController::new(n_rows, n_cols, num_players, n_in_a_row));
    }
}

#[wasm_bindgen]
pub fn get_board() -> Array {
    let rust_board;
    unsafe {
        rust_board = GAME_CONTROLLER.as_ref().unwrap().game.get_board().clone();
    }
    
    let js_board = Array::new();
    for row in rust_board.iter() {
        let js_row = Array::new();
        for &cell in row.iter() {
            js_row.push(&JsValue::from(cell));
        }
        js_board.push(&js_row.into());
    }
    js_board
}

#[wasm_bindgen]
pub fn get_to_move() -> usize {
    unsafe {
        GAME_CONTROLLER.as_ref().unwrap().game.get_to_move()
    }
}

#[wasm_bindgen]
pub fn get_num_moves() -> usize {
    unsafe {
        GAME_CONTROLLER.as_ref().unwrap().game.get_move_num()
    }
}

// Make a move and get the AI's move (for simplicity)
#[wasm_bindgen]
pub fn make_move(col: usize) {
    unsafe {
        let game_controller = GAME_CONTROLLER.as_mut().unwrap();
        game_controller.make_move(col);
    }
}
