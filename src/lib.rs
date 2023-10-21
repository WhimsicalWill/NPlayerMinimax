mod game;
mod tictactoe;
// mod connectfour;
mod pushupfour;
mod eval;
mod opt;

use wasm_bindgen::prelude::*;

// Other imports...
use crate::pushupfour::PushUpFour;
use crate::game::{Game, GameState};
use crate::eval::RandomEvaluationFunction;
use crate::opt::minimax_move;
use serde::{Serialize, Deserialize};

// This is a simple struct to represent the game state in JS-compatible format
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsGameState {
    to_move: usize,
    move_num: usize,
    board: Vec<Vec<i32>>,
}

// Conversion from GameState to JsGameState
impl From<&GameState> for JsGameState {
    fn from(game_state: &GameState) -> Self {
        // Your conversion logic here
        JsGameState {
            to_move: game_state.get_to_move(),
            move_num: game_state.get_move_num(),
            board: game_state.get_board().clone(),
        }
    }
}
// Expose a PushUpFour struct to JS.
#[wasm_bindgen]
pub struct PushUpFourGame {
    game: PushUpFour,
    eval_function: RandomEvaluationFunction,
    search_depth: usize,
}

#[wasm_bindgen]
impl PushUpFourGame {
    #[wasm_bindgen(constructor)]
    pub fn new(n_rows: usize, n_cols: usize, num_players: usize, n_in_a_row: usize) -> PushUpFourGame {
        PushUpFourGame {
            game: PushUpFour::new(n_rows, n_cols, num_players, n_in_a_row),
            eval_function: RandomEvaluationFunction::new(num_players),
            search_depth: 5,
        }
    }

    // Make a move and get the AI's move (for simplicity)
    pub fn make_move(&mut self, col: usize) -> JsGameState {
        self.game.transition(col);

        // AI's move logic, using minimax_move or any other method you have
        let (_, ai_move) = minimax_move(&mut self.game, &self.eval_function, self.search_depth);
        self.game.transition(ai_move);

        let js_game_state = JsGameState::from(self.game.get_state());
        js_game_state
    }

    pub fn get_state(&self) -> JsGameState {
        let js_game_state = JsGameState::from(self.game.get_state());
        js_game_state
    }
}