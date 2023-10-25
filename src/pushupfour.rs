use std::io::{self, Write};
use crate::game::{GameState, Game};

const SYMBOLS: [&str; 4] = ["A", "B", "C", "D"];

pub struct PushUpFour {
    state: GameState,
    num_rows: usize,
    num_cols: usize,
    num_players: usize,
    n_in_a_row: usize,
}

impl PushUpFour {
    pub fn new(num_rows: usize, num_cols: usize, num_players: usize, n_in_a_row: usize) -> Self {
        PushUpFour {
            state: GameState::new(0, 0, vec![vec![-1; num_cols]; num_rows]),
            num_rows,
            num_cols,
            num_players,
            n_in_a_row,
        }
    }

    pub fn get_board(&self) -> &Vec<Vec<i32>> {
        &self.state.get_board()
    }

    pub fn get_to_move(&self) -> usize {
        self.state.get_to_move()
    }

    pub fn get_move_num(&self) -> usize {
        self.state.get_move_num()
    }

    pub fn get_num_players(&self) -> usize {
        self.num_players
    }
    
    pub fn is_sequence_win(&self, sequence: &[i32], player: usize) -> bool {
        sequence.iter().filter(|&&x| x == player as i32).count() >= self.n_in_a_row
    }

    pub fn is_row_win(&self, player: usize) -> bool {
        for row in self.state.get_board().iter() {
            for i in 0..self.num_cols - self.n_in_a_row + 1 {
                if self.is_sequence_win(&row[i..i+self.n_in_a_row], player) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_col_win(&self, player: usize) -> bool {
        for col in 0..self.num_cols {
            let col_elems: Vec<i32> = self.state.get_board().iter().map(|row| row[col]).collect();
            for i in 0..self.num_rows - self.n_in_a_row + 1 {
                if self.is_sequence_win(&col_elems[i..i+self.n_in_a_row], player) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_diag_win(&self, player: usize) -> bool {
        // Check main diagonals
        for i in 0..self.num_rows - self.n_in_a_row + 1 {
            for j in 0..self.num_cols - self.n_in_a_row + 1 {
                let diagonal: Vec<i32> = (0..self.n_in_a_row).map(|k| self.state.get_board()[i+k][j+k]).collect();
                if self.is_sequence_win(&diagonal, player) {
                    return true;
                }
            }
        }

        // Check counter-diagonals
        for i in 0..self.num_rows - self.n_in_a_row + 1 {
            for j in self.n_in_a_row-1..self.num_cols {
                let diagonal: Vec<i32> = (0..self.n_in_a_row).map(|k| self.state.get_board()[i+k][j-k]).collect();
                if self.is_sequence_win(&diagonal, player) {
                    return true;
                }
            }
        }
        false
    }

    fn is_tie_after_transition(&self) -> bool {
        // Check if more than one player wins after a move
        let winners: Vec<bool> = (0..self.num_players).map(|player| self.is_win(player)).collect();
        winners.iter().filter(|&&x| x).count() > 1
    }
}

impl Game for PushUpFour {
    fn get_game_status(&self) -> i32 {
        if self.is_tie() {
            return -1;
        }
        for player in 0..self.num_players {
            if self.is_win(player) {
                return player as i32;
            }
        }
        -2
    }

    fn get_score(&self) -> Vec<f64> {
        let res = self.get_game_status();
        if res < 0 {
            return vec![1.0 / self.num_players as f64; self.num_players];
        } else {
            let mut score = vec![0.0; self.num_players];
            score[res as usize] = 1.0;
            score
        }
    }

    fn get_state(&self) -> &GameState {
        &self.state
    }

    fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    fn get_valid_moves(&self) -> Vec<usize> {
        (0..self.num_cols).filter(|&col| self.state.get_board()[0][col] == -1).collect()
    }

    fn transition(&mut self, move_col: usize) {
        let mut board_copy = self.state.get_board().clone();
        
        // Push the new chip up the bottom, shifting other chips up
        for row in 0..self.num_rows - 1 {
            board_copy[row][move_col] = board_copy[row + 1][move_col];
        }
        board_copy[self.num_rows - 1][move_col] = self.state.get_to_move() as i32;
        
        self.state = GameState::new(
            (self.state.get_to_move() + 1) % self.num_players, 
            self.state.get_move_num() + 1, 
            board_copy
        );
    }

    fn is_win(&self, player: usize) -> bool {
        self.is_row_win(player) || self.is_col_win(player) || self.is_diag_win(player)
    }

    fn is_tie(&self) -> bool {
        self.state.get_move_num() == self.num_rows * self.num_cols || self.is_tie_after_transition()
    }

    fn get_user_move(&self) -> usize {
        loop {
            print!("Enter col (1-7): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let move_col = input.trim().parse::<usize>().unwrap_or(0) - 1;

            if self.get_valid_moves().contains(&move_col) {
                return move_col;
            } else {
                println!("Invalid move, try again");
            }
        }
    }

    fn print(&self) {
        for row in self.state.get_board() {
            for &cell in row {
                let symbol = if cell >= 0 && (cell as usize) < self.num_players {
                    SYMBOLS[cell as usize]
                } else {
                    " "
                };
                print!("| {} ", symbol);
            }
            println!("|");
            for _ in 0..self.num_cols {
                print!("----");
            }
            println!();
        }
        println!("\nPlayer {}'s turn\n", SYMBOLS[self.state.get_to_move()]);
    }
}

