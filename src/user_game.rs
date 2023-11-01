use crate::gametraits::{ValidMoves, TransitionFunction, WinCondition, TieCondition};
use crate::game::{Game, GameState};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1)
];

pub struct UserGameInitialBoard;
impl InitialBoard for UserGameInitialBoard {
    fn initial_board(&self, rows: usize, cols: usize) -> Vec<Vec<i32>> {
        let mut board = vec![vec![-1; cols]; rows];
        let mid_row = rows / 2;
        let mid_col = cols / 2;
        board[mid_row - 1][mid_col - 1] = 0;
        board[mid_row][mid_col] = 0;
        board[mid_row - 1][mid_col] = 1;
        board[mid_row][mid_col - 1] = 1;
        board
    }
}

pub struct UserGameValidMoves;
impl ValidMoves for UserGameValidMoves {
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        let board = game.get_state().get_board();
        let player = game.get_state().get_to_move() as i32;

        for row in 0..8 {
            for col in 0..8 {
                if board[row][col] == -1 && self.is_valid_move(board, player, row, col) {
                    valid_moves.push((row, col));
                }
            }
        }

        valid_moves
    }
}

impl UserGameValidMoves {
    fn is_valid_move(&self, board: &Vec<Vec<i32>>, player: i32, row: usize, col: usize) -> bool {
        DIRECTIONS.iter().any(|&(dx, dy)| {
            self.can_capture_in_direction(board, player, row as isize, col as isize, dx, dy)
        })
    }

    fn can_capture_in_direction(
        &self, 
        board: &Vec<Vec<i32>>, 
        player: i32, 
        row: isize, 
        col: isize, 
        dx: isize, 
        dy: isize
    ) -> bool {
        let mut r = row + dx;
        let mut c = col + dy;
        let mut opponent_seen = false;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            match board[r as usize][c as usize] {
                -1 => return false,
                color if color == player => return opponent_seen,
                _ => opponent_seen = true,
            }

            r += dx;
            c += dy;
        }

        false
    }
}

pub struct UserGameTransitionFunction;
impl TransitionFunction for UserGameTransitionFunction {
    fn transition(&self, game: &Game, move_row: usize, move_col: usize) -> GameState {
        let mut board_copy = game.get_state().get_board().clone();
        let player = game.get_state().get_to_move() as i32;
        board_copy[move_row][move_col] = player;

        for &(dx, dy) in DIRECTIONS.iter() {
            if UserGameValidMoves.is_valid_move(&board_copy, player, move_row, move_col) {
                self.flip_discs(&mut board_copy, player, move_row as isize, move_col as isize, dx, dy);
            }
        }

        GameState::new(
            (player as usize + 1) % 2, 
            game.get_state().get_move_num() + 1, 
            board_copy
        )
    }
}

impl UserGameTransitionFunction {
    fn flip_discs(
        &self,
        board: &mut Vec<Vec<i32>>,
        player: i32,
        row: isize,
        col: isize,
        dx: isize,
        dy: isize
    ) {
        let mut r = row + dx;
        let mut c = col + dy;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            if board[r as usize][c as usize] == player {
                break;
            }

            r += dx;
            c += dy;
        }

        if !(r >= 0 && r < 8 && c >= 0 && c < 8) {
            return;
        }

        let mut r = row + dx;
        let mut c = col + dy;
        while r != row || c != col {
            board[r as usize][c as usize] = player;
            r -= dx;
            c -= dy;
        }
    }
}

pub struct UserGameWinCondition;
impl WinCondition for UserGameWinCondition {
    fn is_win(&self, game: &Game, player: usize) -> bool {
        let board = game.get_state().get_board();
        let player_count = board.iter().flatten().filter(|&&p| p == player as i32).count();
        let opponent_count = board.iter().flatten().filter(|&&p| p == ((player + 1) % 2) as i32).count();

        player_count > opponent_count
    }
}

pub struct UserGameTieCondition;
impl TieCondition for UserGameTieCondition {
    fn is_tie(&self, game: &Game) -> bool {
        let board = game.get_state().get_board();
        let empty_count = board.iter().flatten().filter(|&&p| p == -1).count();

        empty_count == 0 && 
        UserGameWinCondition.is_win(game, 0) == 
        UserGameWinCondition.is_win(game, 1)
    }
}
