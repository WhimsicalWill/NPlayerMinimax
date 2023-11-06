mod eval;
mod game;
mod game_elements;
mod game_spec;
mod opt;
mod user_game;

use crate::eval::RandomEvaluationFunction;
use crate::game::Game;
use crate::game_elements::{GameStatus, Player};
use crate::opt::minimax_move;
use crate::user_game::UserGameSpec;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameController {
    game: Game,
    eval_function: RandomEvaluationFunction,
}

#[wasm_bindgen]
pub fn create_game_controller(num_players: usize) -> GameController {
    let game: Game = Game::new(Box::new(UserGameSpec), num_players);
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
                    None => "",
                    Some(Player::Player0) => "X",
                    Some(Player::Player1) => "O",
                    Some(Player::Player2) => "Z",
                    Some(Player::Player3) => "W",
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
        let (_, move_option) = minimax_move(&mut self.game, &self.eval_function, SEARCH_DEPTH);

        if let Some((move_row, move_col)) = move_option {
            // Execute the move
            self.game.transition(move_row, move_col);
        } else {
            // Handle the case where there's no move
            println!("No valid AI move found");
        }
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