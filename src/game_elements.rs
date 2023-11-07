use std::hash::Hash;

use wasm_bindgen::prelude::*;

#[derive(PartialEq, Copy, Clone, Hash)]
#[wasm_bindgen]
pub enum Player {
    Player0,
    Player1,
    Player2,
    Player3,
}

impl From<usize> for Player {
    fn from(num: usize) -> Self {
        match num {
            0 => Player::Player0,
            1 => Player::Player1,
            2 => Player::Player2,
            3 => Player::Player3,
            _ => panic!("Invalid player number"),
        }
    }
}

impl Player {
    pub fn to_usize(&self) -> usize {
        match self {
            Player::Player0 => 0,
            Player::Player1 => 1,
            Player::Player2 => 2,
            Player::Player3 => 3,
        }
    }
}

#[derive(PartialEq)]
#[wasm_bindgen]
pub enum GameStatus {
    Player0Win,
    Player1Win,
    Player2Win,
    Player3Win,
    Ongoing,
    Tie,
}

pub type BoardCell = Option<Player>;
