impl GameState {
    pub fn new(to_move: usize, move_num: usize, board: Vec<Vec<i32>>) -> Self {
        GameState {
            to_move,
            move_num,
            board,
        }
    }

    pub fn get_to_move(&self) -> usize {
        self.to_move
    }

    pub fn get_move_num(&self) -> usize {
        self.move_num
    }

    pub fn get_board(&self) -> &Vec<Vec<i32>> {
        &self.board
    }
}

pub struct Game {
    state: GameState,
    num_rows: usize,
    num_cols: usize,
    num_players: usize,
    n_in_a_row: usize,
    move_validator: Box<dyn MoveValidator>,
    game_transition: Box<dyn GameTransition>,
    win_condition: Box<dyn WinCondition>,
    tie_condition: Box<dyn TieCondition>,
}

impl Game {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        num_players: usize,
        n_in_a_row: usize,
        move_validator: Box<dyn MoveValidator>,
        game_transition: Box<dyn GameTransition>,
        win_condition: Box<dyn WinCondition>,
        tie_condition: Box<dyn TieCondition>,
    ) -> Game {
        Game {
            state: GameState::new(0, 0, vec![vec![-1; num_cols]; num_rows]),
            num_rows,
            num_cols,
            num_players,
            n_in_a_row,
            move_validator,
            game_transition,
            win_condition,
            tie_condition,
        }
    }

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

    fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    // // TODO: set via dependency injection
    // fn get_valid_moves(&self) -> Vec<usize> {
    //     (0..self.num_cols).filter(|&col| self.state.get_board()[0][col] == -1).collect()
    // }

    // // TODO: set via dependency injection
    // fn transition(&mut self, move_col: usize) {
    //     let mut board_copy = self.state.get_board().clone();
        
    //     // Push the new chip up the bottom, shifting other chips up
    //     for row in 0..self.num_rows - 1 {
    //         board_copy[row][move_col] = board_copy[row + 1][move_col];
    //     }
    //     board_copy[self.num_rows - 1][move_col] = self.state.get_to_move() as i32;
        
    //     self.state = GameState::new(
    //         (self.state.get_to_move() + 1) % self.num_players, 
    //         self.state.get_move_num() + 1, 
    //         board_copy
    //     );
    // }

    // // TODO: set via dependency injection
    // fn is_win(&self, player: usize) -> bool {
    //     self.is_row_win(player) || self.is_col_win(player) || self.is_diag_win(player)
    // }

    // // TODO: set via dependency injection
    // fn is_tie(&self) -> bool {
    //     self.state.get_move_num() == self.num_rows * self.num_cols || self.is_tie_after_transition()
    // }

    // Functions that call the dependency injected functions
    fn get_valid_moves(&self) -> Vec<usize> {
        self.move_validator.get_valid_moves(self)
    }

    fn transition(&mut self, move_col: usize) {
        self.game_transition.transition(self, move_col);
    }

    fn is_win(&self, player: usize) -> bool {
        self.win_condition.is_win(self, player)
    }

    fn is_tie(&self) -> bool {
        self.tie_condition.is_tie(self)
    }
}

