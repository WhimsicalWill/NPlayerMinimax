use crate::game_traits::{InitialState, ValidMoves, TransitionFunction, WinCondition, TieCondition};
use crate::game_elements::{Player, GameStatus, BoardCell};

// TODO: why do we need Clone here and not just copy (if all of the fields are copyable?)
#[derive(Clone)]
pub struct GameState {
    to_move: Player,
    move_num: usize,
    board: Vec<Vec<BoardCell>>,
}

impl GameState {
    pub fn new(to_move: Player, move_num: usize, board: Vec<Vec<BoardCell>>) -> Self {
        GameState {
            to_move,
            move_num,
            board,
        }
    }

    pub fn get_to_move(&self) -> Player {
        self.to_move
    }

    pub fn get_move_num(&self) -> usize {
        self.move_num
    }

    pub fn get_board(&self) -> &Vec<Vec<BoardCell>> {
        &self.board
    }
}

pub struct Game {
    state: GameState,
    num_rows: usize,
    num_cols: usize,
    num_players: usize,
    initial_state: Box<dyn InitialState>, // New way to inject initial board
    valid_moves: Box<dyn ValidMoves>,
    transition_function: Box<dyn TransitionFunction>,
    win_condition: Box<dyn WinCondition>,
    tie_condition: Box<dyn TieCondition>,
}

impl Game {
    pub fn new(
        num_rows,
        num_cols,
        num_players: usize,
        initial_state: Box<dyn InitialState>,
        valid_moves: Box<dyn ValidMoves>,
        transition_function: Box<dyn TransitionFunction>,
        win_condition: Box<dyn WinCondition>,
        tie_condition: Box<dyn TieCondition>,
    ) -> Game {
        Game {
            state: GameState::new(initial_state.get_to_move(), initial_state.get_move_num(), initial_state.get_board()),
            num_rows,
            num_cols,
            num_players,
            initial_state,
            valid_moves,
            transition_function,
            win_condition,
            tie_condition,
        }
    }

    pub fn get_game_status(&self) -> GameStatus {
        if self.is_tie() {
            return GameStatus::Tie;
        }
        for player in 0..self.num_players {
            if self.is_win(player) {
                return GameStatus::Win(Player::from(player));
            }
        }
        return GameStatus::Ongoing;
    }

    pub fn get_score(&self) -> Vec<f64> {
        match self.get_game_status() {
            GameStatus::Tie => vec![1.0 / self.num_players as f64; self.num_players],
            GameStatus::Win(player) => {
                let mut score = vec![0.0; self.num_players];
                score[player.to_usize()] = 1.0;
                score
            }
            _ => vec![0.0; self.num_players],
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn get_num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn get_num_players(&self) -> usize {
        self.num_players
    }

    pub fn get_board(&self) -> &Vec<Vec<BoardCell>> {
        self.state.get_board()
    }

    pub fn get_to_move(&self) -> Player {
        self.state.get_to_move()
    }

    pub fn get_move_num(&self) -> usize {
        self.state.get_move_num()
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    // Below: functions that call the dependency injected functions
    pub fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        self.valid_moves.get_valid_moves(self)
    }

    pub fn transition(&mut self, move_row: usize, move_col: usize) {
        self.state = self.transition_function.transition(self, move_row, move_col);
    }

    pub fn is_win(&self, player: Player) -> bool {
        self.win_condition.is_win(self, player)
    }

    pub fn is_tie(&self) -> bool {
        self.tie_condition.is_tie(self)
    }
}