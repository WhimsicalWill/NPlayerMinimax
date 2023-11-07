use crate::game_elements::{BoardCell, GameStatus, Player};
use crate::game_spec::GameSpec;

#[derive(Clone)]
pub struct GameState {
    to_move: Player,
    move_num: usize,
    board: Vec<Vec<BoardCell>>,
    prev_state: Option<Box<GameState>>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.to_move == other.to_move &&
        self.move_num == other.move_num &&
        self.board == other.board
        // Note that we do not compare prev_state
    }
}
impl Eq for GameState {}

impl GameState {
    pub fn new(to_move: Player, move_num: usize, board: Vec<Vec<BoardCell>>) -> Self {
        GameState {
            to_move,
            move_num,
            board,
            prev_state: None,
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

    pub fn get_prev_state(&self) -> &Option<Box<GameState>> {
        &self.prev_state
    }

    pub fn set_prev_state(&mut self, state: Box<GameState>) {
        self.prev_state = Some(state);
    }
}

pub struct Game {
    state: Box<GameState>,
    spec: Box<dyn GameSpec>,
    num_players: usize,
}

impl Game {
    pub fn new(
        spec: Box<dyn GameSpec>,
        num_players: usize,
    ) -> Game {
        Game {
            state: Box::new(GameState::new(
                spec.get_initial_to_move(),
                0,
                spec.get_initial_board(),
            )),
            spec,
            num_players,
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

    fn create_score_vector(&self, winning_player: Player) -> Vec<f64> {
        let winning_player_idx = winning_player.to_usize();
        let mut score = vec![0.0; self.num_players];
        score[winning_player_idx] = 1.0;
        score
    }

    // TODO: gracefully apply an injected eval function to the state (for ongoing game)
    pub fn get_score(&self) -> Vec<f64> {
        match self.get_game_status() {
            GameStatus::Tie | GameStatus::Ongoing => {
                vec![1.0 / self.num_players as f64; self.num_players]
            }
            GameStatus::Player0Win => self.create_score_vector(Player::Player0),
            GameStatus::Player1Win => self.create_score_vector(Player::Player1),
            GameStatus::Player2Win => self.create_score_vector(Player::Player2),
            GameStatus::Player3Win => self.create_score_vector(Player::Player3),
        }
    }

     pub fn undo_transition(&mut self) {
        if let Some(prev_state) = self.state.prev_state.take() {
            self.state = prev_state;
        } else {
            panic!("Attempted to undo initial state");
        }
    }

    pub fn get_state(&self) -> &Box<GameState> {
        &self.state
    }

    pub fn get_num_players(&self) -> usize {
        self.num_players
    }

    pub fn get_next_player(&self) -> Player {
        let next_player_num = (self.get_to_move().to_usize() + 1) % self.get_num_players();
        Player::from(next_player_num)
    }

    pub fn get_to_move(&self) -> Player {
        self.state.get_to_move()
    }

    pub fn get_move_num(&self) -> usize {
        self.state.get_move_num()
    }

    pub fn get_board(&self) -> &Vec<Vec<BoardCell>> {
        self.state.get_board()
    }

    pub fn get_prev_state(&self) -> &Option<Box<GameState>> {
        self.state.get_prev_state()
    }

    /*
    ----------The functions below call the functions in the game spec----------
    */

    pub fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        self.spec.get_valid_moves(self)
    }

    pub fn transition(&mut self, move_row: usize, move_col: usize) {
        // Other state is (initially) the state after the move is made
        let mut other_state = self.spec.transition(self, move_row, move_col);

        // We use std::mem::swap to exchange self.state with other_state directly
        std::mem::swap(&mut self.state, &mut other_state);

        // After swapping, other_state contains the previous state
        // We now set the previous state of self.state to be other_state
        self.state.set_prev_state(other_state);
    }

    pub fn is_win(&self, player: Player) -> bool {
        self.spec.is_win(self, player)
    }

    pub fn is_tie(&self) -> bool {
        self.spec.is_tie(self)
    }
}
