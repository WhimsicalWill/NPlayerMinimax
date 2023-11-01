use wasm_bindgen::prelude::*;

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
    fn to_usize(&self) -> usize {
        match self {
            Player::Player0 => 0,
            Player::Player1 => 1,
            Player::Player2 => 2,
            Player::Player3 => 3,
        }
    }
}

#[wasm_bindgen]
pub enum GameStatus {
    Ongoing,
    Tie,
    Win(Player),
}

pub enum BoardCell {
    Empty,
    Occupied(Player),
}
