use std::panic::panic_any;
use crate::{ChessBoard, ChessError};
use crate::core::{Move, Position};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameStatus {
    Ongoing,
    Checkmate,
    Stalemate,
}

pub struct GameState<'a> {
    pub status: GameStatus,
    board: &'a ChessBoard,
    available_moves: Vec<Move>
}

/// A simple chess engine that manages the chess board and handles moves.
#[derive(Default)]
pub struct ChessEngine {
    chess_board: ChessBoard,
    moves: Vec<Move>,
}

impl ChessEngine {
    /// Get the current chess board.
    pub fn get_board(&self) -> &ChessBoard {
        &self.chess_board
    }

    /// Try and make a move on the chess board.
    pub fn make_move(&mut self, from: Position, to: Position) -> Result<GameState<'_>, ChessError> {
        self.moves.push(Move {
            from,
            to,
        });
        if !self.get_available_moves(from)?.contains(&to) {
            Err(ChessError::InvalidMove("".to_string()))
        } else {
            // TODO: implement move logic here and handle taking pieces, etc....
            Ok(GameState {
                status: GameStatus::Ongoing,
                board: &self.chess_board,
                // TODO: we need to get all the available moves for the piece at `from`
                available_moves: vec![],
            })
        }
    }

    /// Get the available moves for a piece at the given position.
    pub fn get_available_moves(&self, piece: Position) -> Result<Vec<Position>, ChessError> {
        panic!("get_available_moves not implemented yet");
    }

    fn available_move_for_pawn(&self, from: Position) -> Result<Vec<Position>, ChessError> {
        panic!("available_move_for_pawn not implemented yet");
    }
}