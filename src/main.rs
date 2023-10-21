mod game;
mod tictactoe;
// mod connectfour;
mod pushupfour;
mod eval;
mod opt;

use std::time::Instant;
// use crate::connectfour::ConnectFour;
use crate::pushupfour::PushUpFour;
use crate::game::Game;
use crate::opt::minimax_move;
use crate::eval::RandomEvaluationFunction;

fn main() {
    let num_players = 2;
    let n_rows = 6;
    let n_cols = 7;
    let n_in_a_row = 4;
    let search_depth = 6;
    let human_player = 0;  // human plays as player 0 (A)
    // let mut game = ConnectFour::new(n_rows, n_cols, num_players, n_in_a_row);
    let mut game = PushUpFour::new(n_rows, n_cols, num_players, n_in_a_row);
    let eval_function = RandomEvaluationFunction::new(num_players);


    game.print();

    while game.get_game_status() == -2 {
        let action;
        if game.get_to_move() == human_player {
            action = game.get_user_move();
        } else {
            let start_time = Instant::now();
            let (_, best_move) = minimax_move(&mut game, &eval_function, search_depth);
            let duration = start_time.elapsed();
            println!("AI's move took {:.2?} seconds.", duration);

            action = best_move;
        }

        game.transition(action);
        game.print();
    }

    println!("Game over: {}", game.get_game_status());
}