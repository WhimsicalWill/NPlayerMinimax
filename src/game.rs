use crate::game_elements::{BoardCell, GameStatus, Player};
use crate::game_traits::{
    InitialState, TieCondition, TransitionFunction, ValidMoves, WinCondition,
};

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
    valid_moves: Box<dyn ValidMoves>,
    transition_function: Box<dyn TransitionFunction>,
    win_condition: Box<dyn WinCondition>,
    tie_condition: Box<dyn TieCondition>,
}

impl Game {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        num_players: usize,
        initial_state: Box<dyn InitialState>,
        valid_moves: Box<dyn ValidMoves>,
        transition_function: Box<dyn TransitionFunction>,
        win_condition: Box<dyn WinCondition>,
        tie_condition: Box<dyn TieCondition>,
    ) -> Game {
        Game {
            state: GameState::new(
                initial_state.get_to_move(),
                initial_state.get_move_num(),
                initial_state.get_board(num_rows, num_cols),
            ),
            num_rows,
            num_cols,
            num_players,
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
        for player_idx in 0..self.num_players {
            if self.is_win(Player::from(player_idx)) {
                return match player_idx {
                    0 => GameStatus::Player0Win,
                    1 => GameStatus::Player1Win,
                    2 => GameStatus::Player2Win,
                    3 => GameStatus::Player3Win,
                    _ => panic!("Invalid player number"),
                };
            }
        }
        return GameStatus::Ongoing;
    }

    fn create_score_vector(winning_player_index: usize, num_players: usize) -> Vec<f64> {
        let mut score = vec![0.0; num_players];
        score[winning_player_index] = 1.0;
        score
    }

    // TODO: gracefully apply an injected eval function to the state (for ongoing game)
    pub fn get_score(&self) -> Vec<f64> {
        match self.get_game_status() {
            GameStatus::Tie | GameStatus::Ongoing => {
                vec![1.0 / self.num_players as f64; self.num_players]
            }
            GameStatus::Player0Win => {
                Game::create_score_vector(Player::Player0.to_usize(), self.num_players)
            }
            GameStatus::Player1Win => {
                Game::create_score_vector(Player::Player1.to_usize(), self.num_players)
            }
            GameStatus::Player2Win => {
                Game::create_score_vector(Player::Player2.to_usize(), self.num_players)
            }
            GameStatus::Player3Win => {
                Game::create_score_vector(Player::Player3.to_usize(), self.num_players)
            }
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

    pub fn get_next_player(&self) -> Player {
        let next_player_num = (self.get_to_move().to_usize() + 1) % self.get_num_players();
        Player::from(next_player_num)
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
        self.state = self
            .transition_function
            .transition(self, move_row, move_col);
    }

    pub fn is_win(&self, player: Player) -> bool {
        self.win_condition.is_win(self, player)
    }

    pub fn is_tie(&self) -> bool {
        self.tie_condition.is_tie(self)
    }
}
