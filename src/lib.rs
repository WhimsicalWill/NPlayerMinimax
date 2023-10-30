mod game;
mod pushupfour;
mod go;
mod eval;
mod opt;
mod gametraits;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::pushupfour::{PushUpFourAvailableMoves, PushUpFourGameTransition, PushUpFourWinCondition, PushUpFourTieCondition};
use crate::go::{GoAvailableMoves, GoGameTransition, GoWinCondition, GoTieCondition};
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
        Box::new(PushUpFourAvailableMoves {}),
        Box::new(PushUpFourGameTransition {}),
        Box::new(PushUpFourWinCondition {}),
        Box::new(PushUpFourTieCondition {}),
        // Box::new(GoAvailableMoves {}),
        // Box::new(GoGameTransition {}),
        // Box::new(GoWinCondition {}),
        // Box::new(GoTieCondition {}),
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
    let valid_moves = self.game.get_valid_human_moves();
    
    let move_is_valid = valid_moves.iter().any(
        |&(row, col)| row == move_row && col == move_col
    );

    if !move_is_valid {
        return; // TODO: return a status code
    }
    
    self.game.transition(move_row, move_col);
}

}