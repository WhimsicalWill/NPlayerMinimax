use crate::game::{Game, GameState};
use crate::game_elements::{BoardCell, Player};
use crate::game_spec::GameSpec;

pub struct OthelloSpec;

impl OthelloSpec {
    fn flip_discs(board: &mut Vec<Vec<BoardCell>>, row: usize, col: usize, player: Player) {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for dir in directions.iter() {
            let mut r = row as i32 + dir.0;
            let mut c = col as i32 + dir.1;

            let mut pieces_to_flip = Vec::new();

            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                match board[r as usize][c as usize] {
                    Some(p) if p != player => pieces_to_flip.push((r as usize, c as usize)),
                    Some(p) if p == player => {
                        for &(flip_r, flip_c) in &pieces_to_flip {
                            board[flip_r][flip_c] = Some(player);
                        }
                        break;
                    }
                    _ => break,
                }

                r += dir.0;
                c += dir.1;
            }
        }
    }

    fn is_valid_move(board: &Vec<Vec<BoardCell>>, row: usize, col: usize, player: Player) -> bool {
        if board[row][col].is_some() {
            return false;
        }

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for dir in directions.iter() {
            let mut r = row as i32 + dir.0;
            let mut c = col as i32 + dir.1;

            let mut has_opponent_between = false;

            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                match board[r as usize][c as usize] {
                    Some(p) if p != player => has_opponent_between = true,
                    Some(p) if p == player => {
                        return has_opponent_between;
                    }
                    _ => break,
                }

                r += dir.0;
                c += dir.1;
            }
        }

        false
    }

    fn get_player_disc_count(&self, game: &Game, player: Player) -> usize {
        game.get_board()
            .iter()
            .flatten()
            .filter(|&cell| *cell == Some(player))
            .count()
    }
}

impl GameSpec for OthelloSpec {
    fn get_initial_board(&self, _num_rows: usize, _num_cols: usize) -> Vec<Vec<BoardCell>> {
        let mut board = vec![vec![None; 8]; 8];
        board[3][3] = Some(Player::Player0);
        board[4][4] = Some(Player::Player0);
        board[3][4] = Some(Player::Player1);
        board[4][3] = Some(Player::Player1);
        board
    }

    fn get_initial_to_move(&self) -> Player {
        Player::Player0
    }

    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if Self::is_valid_move(game.get_board(), row, col, game.get_to_move()) {
                    valid_moves.push((row, col));
                }
            }
        }
        valid_moves
    }

    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> GameState {
        let mut board_copy = game.get_board().clone();
        board_copy[move_row][move_col] = Some(game.get_to_move());
        Self::flip_discs(&mut board_copy, move_row, move_col, game.get_to_move());
        GameState::new(game.get_next_player(), game.get_move_num() + 1, board_copy)
    }

    fn is_win(&self, game: &Game, player: Player) -> bool {
        // Win if opponent has no valid moves and the current player has more discs
        let opponent = if player == Player::Player0 {
            Player::Player1
        } else {
            Player::Player0
        };
        if self.get_valid_moves(game).is_empty() && self.get_player_disc_count(game, player) > self
            .get_player_disc_count(game, opponent)
        {
            return true;
        }
        false
    }

    fn is_tie(&self, game: &Game) -> bool {
        // Tie if neither player can move
        self.get_valid_moves(game).is_empty()
            && self.get_player_disc_count(game, Player::Player0)
                == self.get_player_disc_count(game, Player::Player1)
    }
}
