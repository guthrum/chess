use crate::core::{Cell, ChessPiece, ChessPieceKind, Move, Position};
use crate::{ChessBoard, ChessError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameStatus {
    Ongoing,
    Checkmate,
    Stalemate,
}

pub struct GameState<'a> {
    pub status: GameStatus,
    board: &'a ChessBoard,
}

/// A simple chess engine that manages the chess board and handles moves.
#[derive(Default)]
pub struct ChessEngine {
    chess_board: ChessBoard,
    moves: Vec<Move>,
    taken_pieces: Vec<ChessPiece>,
}

impl ChessEngine {
    /// Get the current chess board.
    pub fn get_board(&self) -> &ChessBoard {
        &self.chess_board
    }

    /// Try and make a move on the chess board.
    pub fn make_move(&mut self, from: Position, to: Position) -> Result<GameState<'_>, ChessError> {
        let from_cell = self
            .chess_board
            .get_piece_at(&from)
            .ok_or(ChessError::InvalidMove(
                "from position does not exist".to_string(),
            ))?;
        if from_cell.piece.is_none() {
            return Err(ChessError::InvalidMove(
                "no piece at from position".to_string(),
            ));
        }
        if from_cell.piece.as_ref().unwrap().colour != self.chess_board.turn {
            return Err(ChessError::InvalidMove(
                "cannot move opponent's piece".to_string(),
            ));
        }

        if !self.get_available_moves(from)?.contains(&to) {
            Err(ChessError::InvalidMove("".to_string()))
        } else {
            self.moves.push(Move { from, to });

            // Move the piece on the board
            let (old_x, old_y) = from.board_position();
            let (new_x, new_y) = to.board_position();
            let taken_cell = self.chess_board.board[new_y][new_x];
            let old_cell = self.chess_board.board[old_y][old_x];
            self.chess_board.board[new_y][new_x] = Cell {
                piece: old_cell.piece,
                colour: taken_cell.colour,
            };

            self.chess_board.board[old_y][old_x] = Cell {
                piece: None,
                colour: old_cell.colour,
            };
            if let Some(taken_piece) = taken_cell.piece {
                self.taken_pieces.push(taken_piece);
            }
            self.chess_board.turn = self.chess_board.turn.flip();

            Ok(GameState {
                status: GameStatus::Ongoing,
                board: &self.chess_board,
            })
        }
    }

    /// Get the available moves for a piece at the given position.
    pub fn get_available_moves(&self, pos: Position) -> Result<Vec<Position>, ChessError> {
        let cell = self
            .chess_board
            .get_piece_at(&pos)
            .ok_or(ChessError::InvalidMove("cell does not exist".to_string()))?;
        if let Some(piece) = cell.piece {
            let raw_moves = match piece.kind {
                ChessPieceKind::Pawn => self.available_move_for_pawn(&pos, &piece),
                ChessPieceKind::Knight => self.available_move_for_knight(&pos, &piece),
                ChessPieceKind::Bishop => Ok(self.available_move_for_bishop(&pos, &piece)),
                ChessPieceKind::Rook => Ok(self.available_move_for_rook(&pos, &piece)),
                ChessPieceKind::Queen => Ok(self.available_move_for_queen(&pos, &piece)),
                ChessPieceKind::King => Ok(self.available_move_for_king(&pos, &piece)),
            }?
            .into_iter()
            .filter(|m| {
                // filter out moves that are not valid because of other pieces
                if let Some(cell) = self.chess_board.get_piece_at(m) {
                    cell.piece.is_none()
                        || cell
                            .piece
                            .map(|p| p.colour != piece.colour)
                            .unwrap_or(false)
                } else {
                    false
                }
            })
            .collect();

            Ok(raw_moves)
        } else {
            Ok(Vec::new())
        }
    }

    fn available_move_for_pawn(
        &self,
        pos: &Position,
        piece: &ChessPiece,
    ) -> Result<Vec<Position>, ChessError> {
        let direction = piece.colour.direction();
        let mut available_moves = Vec::new();
        if let Ok(next_row) = pos.row.try_add(direction) {
            available_moves.push(Position {
                row: next_row,
                column: pos.column,
            })
        }
        if !piece.moved {
            let double_move_row = pos.row.try_add(direction)?.try_add(direction)?;
            available_moves.push(Position {
                row: double_move_row,
                column: pos.column,
            });
        }

        // handle diagonal capture for the pawn
        for i in [-1, 1] {
            if let Ok(diag) = pos.add_offset(direction, i) {
                if let Some(cell) = self.chess_board.get_piece_at(&diag) {
                    if cell
                        .piece
                        .map(|p| p.colour != piece.colour)
                        .unwrap_or(false)
                    {
                        available_moves.push(diag)
                    }
                }
            }
        }

        Ok(available_moves)
    }

    fn available_move_for_king(&self, pos: &Position, piece: &ChessPiece) -> Vec<Position> {
        // TODO: this does not prevent the king moving to be in check
        // TODO: this does not handle castling
        vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(j, i)| pos.add_offset(j, i))
            .flatten()
            .collect()
    }

    fn available_move_for_knight(
        &self,
        pos: &Position,
        piece: &ChessPiece,
    ) -> Result<Vec<Position>, ChessError> {
        Ok([
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ]
        .iter()
        .flat_map(|(x, y)| pos.add_offset(*x, *y))
        .collect())
    }

    fn available_move_for_bishop(&self, pos: &Position, piece: &ChessPiece) -> Vec<Position> {
        vec![(1, 1), (-1, 1), (-1, -1), (1, -1)]
            .into_iter()
            .flat_map(|(i, j)| self.moves_with_offset(pos, i, j))
            .collect()
    }

    fn available_move_for_rook(&self, pos: &Position, piece: &ChessPiece) -> Vec<Position> {
        vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .flat_map(|(i, j)| self.moves_with_offset(pos, i, j))
            .collect()
    }

    fn available_move_for_queen(&self, pos: &Position, piece: &ChessPiece) -> Vec<Position> {
        vec![
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, 1),
            (-1, -1),
            (1, -1),
        ]
        .into_iter()
        .flat_map(|(i, j)| self.moves_with_offset(pos, i, j))
        .collect()
    }

    fn moves_with_offset(&self, pos: &Position, i: isize, j: isize) -> Vec<Position> {
        let mut moves = Vec::new();
        for n in 1..8 {
            if let Ok(npos) = pos.add_offset(i * n, j * n) {
                if let Some(cell) = self.chess_board.get_piece_at(&npos) {
                    moves.push(npos);
                    if cell.piece.is_some() {
                        return moves;
                    }
                }
            }
        }
        moves
    }
}
