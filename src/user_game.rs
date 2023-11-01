// Name: Othello
// InitialState: A board of 8x8 with 2 black and 2 white disks in the center.
// ValidMoves: Players can place a disk on an empty square so that there is at least one straight
//             (horizontal, vertical, or diagonal) occupied line between the new disk and another
//             disk of the player's color, with one or more contiguous opponent disks between them.
// TransitionFunction: Placing a disk flips all opponent disks lying on a straight line between the new disk
//                     and any of the player's previously placed disks.
// WinCondition: The player with the most disks when the board is full, or no valid moves are available for both players.
// TieCondition: The game is a tie if both players have an equal number of disks when the game ends.

use crate::gametraits::{InitialBoard, ValidMoves, TransitionFunction, WinCondition, TieCondition};
use crate::game_elements::{Player, BoardCell};
use crate::game::{Game, GameState};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1)
];

pub struct UserGameInitialBoard;
impl InitialBoard for UserGameInitialBoard {
    fn initial_board(&self, rows: usize, cols: usize) -> Vec<Vec<BoardCell>> {
        let mut board = vec![vec![BoardCell::Empty; cols]; rows];
        let mid_row = rows / 2;
        let mid_col = cols / 2;
        board[mid_row - 1][mid_col - 1] = BoardCell::Occupied(Player::Player0);
        board[mid_row][mid_col] = BoardCell::Occupied(Player::Player0);
        board[mid_row - 1][mid_col] = BoardCell::Occupied(Player::Player1);
        board[mid_row][mid_col - 1] = BoardCell::Occupied(Player::Player1);
        board
    }
}

pub struct UserGameValidMoves;
impl ValidMoves for UserGameValidMoves {
    fn get_valid_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        let board = game.get_state().get_board();
        let player = Player::from(game.get_state().get_to_move());

        for row in 0..8 {
            for col in 0..8 {
                if let BoardCell::Empty = board[row][col] {
                    if self.is_valid_move(board, player, row, col) {
                        valid_moves.push((row, col));
                    }
                }
            }
        }

        valid_moves
    }
}

impl UserGameValidMoves {
    fn is_valid_move(&self, board: &Vec<Vec<BoardCell>>, player: Player, row: usize, col: usize) -> bool {
        DIRECTIONS.iter().any(|&(dx, dy)| {
            self.can_capture_in_direction(board, player, row as isize, col as isize, dx, dy)
        })
    }

    fn can_capture_in_direction(
        &self, 
        board: &Vec<Vec<BoardCell>>, 
        player: Player, 
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
                BoardCell::Empty => return false,
                BoardCell::Occupied(color) if color == player => return opponent_seen,
                BoardCell::Occupied(_) => opponent_seen = true,
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
        let player = Player::from(game.get_state().get_to_move());
        board_copy[move_row][move_col] = BoardCell::Occupied(player);

        for &(dx, dy) in DIRECTIONS.iter() {
            if UserGameValidMoves.is_valid_move(&board_copy, player, move_row, move_col) {
                self.flip_discs(&mut board_copy, player, move_row as isize, move_col as isize, dx, dy);
            }
        }

        GameState::new(
            player.opponent().to_usize(), 
            game.get_state().get_move_num() + 1, 
            board_copy
        )
    }
}

impl UserGameTransitionFunction {
    fn flip_discs(
        &self,
        board: &mut Vec<Vec<BoardCell>>,
        player: Player,
        row: isize,
        col: isize,
        dx: isize,
        dy: isize
    ) {
        let mut r = row + dx;
        let mut c = col + dy;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            match board[r as usize][c as usize] {
                BoardCell::Occupied(color) if color == player => break,
                _ => {},
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
            board[r as usize][c as usize] = BoardCell::Occupied(player);
            r -= dx;
            c -= dy;
        }
    }
}

pub struct UserGameWinCondition;
impl WinCondition for UserGameWinCondition {
    fn is_win(&self, game: &Game, player: Player) -> bool {
        let board = game.get_state().get_board();
        let mut player_count = 0;
        let mut opponent_count = 0;

        for row in board {
            for cell in row {
                match cell {
                    BoardCell::Occupied(p) if p == player => player_count += 1,
                    BoardCell::Occupied(_) => opponent_count += 1,
                    _ => {},
                }
            }
        }

        player_count > opponent_count
    }
}

pub struct UserGameTieCondition;
impl TieCondition for UserGameTieCondition {
    fn is_tie(&self, game: &Game) -> bool {
        let board = game.get_state().get_board();
        for row in board {
            for cell in row {
                if let BoardCell::Empty = cell {
                    return false;
                }
            }
        }
        true
    }
}