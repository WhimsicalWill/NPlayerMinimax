mod game;
mod tictactoe;
mod pushupfour;
mod eval;
mod opt;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::pushupfour::PushUpFour;
use crate::game::Game;
use crate::eval::RandomEvaluationFunction;
use crate::opt::minimax_move;


/*
Using the Game trait, we can have polymorphism in our wasm_bindings
The ControllerBuilder will take in an argument specifying which game to use
It will create the corresponding game, which implements the Game trait
Then, a set of functions will be defined for the ControllerBuilder that can be used in any game

If possible, we want to use wasm_bindgen to expose the Game trait to JS
Then, we can have a factory method that returns a certain game that implements the Game trait
This will also mean that we will just need two extra functions: make_human_move and make_ai_move
*/




pub struct PushUpFourController {
    game: PushUpFour,
    eval_function: RandomEvaluationFunction,
    search_depth: usize,
}

impl PushUpFourController {
    pub fn new(n_rows: usize, n_cols: usize, num_players: usize, n_in_a_row: usize, search_depth: usize) -> PushUpFourController {
        PushUpFourController {
            game: PushUpFour::new(n_rows, n_cols, num_players, n_in_a_row),
            eval_function: RandomEvaluationFunction::new(num_players),
            search_depth,
        }
    }
}

// Static mutable reference to store the game state
static mut CURRENT_GAME: Option<PushUpFourController> = None;

#[wasm_bindgen]
pub fn initialize_game(n_rows: usize, n_cols: usize, num_players: usize, n_in_a_row: usize, search_depth: usize) {
    unsafe {
        CURRENT_GAME = Some(PushUpFourController::new(n_rows, n_cols, num_players, n_in_a_row, search_depth));
    }
}

#[wasm_bindgen]
pub fn get_board() -> Array {
    let rust_board;
    unsafe {
        rust_board = CURRENT_GAME.as_ref().unwrap().game.get_board().clone();
    }
    
    let js_board = Array::new();
    for row in rust_board.iter() {
        let js_row = Array::new();
        for &cell in row.iter() {
            let value = match cell {
                -1 => "",
                0 => "X",
                1 => "O",
                _ => "", // default
            };
            js_row.push(&JsValue::from_str(value));
        }
        js_board.push(&js_row.into());
    }
    js_board
}

#[wasm_bindgen]
pub fn get_to_move() -> usize {
    unsafe {
        CURRENT_GAME.as_ref().unwrap().game.get_to_move()
    }
}

#[wasm_bindgen]
pub fn get_num_moves() -> usize {
    unsafe {
        CURRENT_GAME.as_ref().unwrap().game.get_move_num()
    }
}

#[wasm_bindgen]
pub fn make_human_move(col: usize) {
    let game_controller;
    unsafe {
        game_controller = CURRENT_GAME.as_mut().unwrap();
    }
    game_controller.game.transition(col);
}

#[wasm_bindgen]
pub fn make_ai_move() {
    let game_controller;
    unsafe {
        game_controller = CURRENT_GAME.as_mut().unwrap();
    }
    let (_, ai_move) = minimax_move(&mut game_controller.game, &game_controller.eval_function, game_controller.search_depth);
    game_controller.game.transition(ai_move);
}

#[wasm_bindgen]
pub fn game_status() -> i32 {
    unsafe {
        CURRENT_GAME.as_ref().unwrap().game.get_game_status()
    }
}