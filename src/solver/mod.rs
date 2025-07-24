use crate::core::{ChessBoard, ChessColour, ChessError, ChessPiece, Move, Position};
use crate::game::ChessGame;

type BoardScore = i32;

fn score_board(board: &ChessBoard, self_colour: &ChessColour) -> BoardScore {
    // TODO: this is quite simplistic can improve later

    let mut our_value = 0;
    let mut their_value = 0;
    for piece in board.pieces() {
        match piece.1.piece {
            Some(ref chess_piece) => {
                let value = chess_piece.kind.value() as BoardScore;
                if chess_piece.colour == *self_colour {
                    our_value += value;
                } else {
                    their_value += value;
                }
            }
            None => continue,
        }
    }

    our_value - their_value
}

struct RecursionContext {
    depth: usize,
    start_time: std::time::Instant,
}

impl RecursionContext {
    fn should_recurse(&self) -> bool {
        self.depth < 10 && self.start_time.elapsed().as_secs() <= 5
    }

    fn recurse(&self) -> Self {
        Self {
            depth: self.depth + 1,
            start_time: self.start_time,
        }
    }
}

struct MoveState {
    // TODO: we might want to track the history of the score to see if this is a spiral of doom...
    move_: Move,
    score: BoardScore,
    game: ChessGame,
}

fn best_move_from_position(
    game: &ChessGame,
    self_colour: &ChessColour,
    context: RecursionContext,
) -> Option<(Move, BoardScore)> {
    if !context.should_recurse() {
        return None;
    }
    let movable_pieces: Vec<(Position, ChessPiece)> = game
        .get_board()
        .pieces()
        .filter_map(|(position, cell)| match &cell.piece {
            Some(piece) => {
                if piece.colour == *self_colour {
                    Some((position, *piece))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    // rough approx of what we should pre-allocation
    let mut move_options = Vec::with_capacity(movable_pieces.len() * 4);

    for (position, piece) in movable_pieces {
        if let Ok(moves) = game.get_available_moves(position) {
            for to_pos in moves {
                let mut new_game = game.clone();
                let mv = Move {
                    from: position,
                    to: to_pos,
                };
                if let Ok(state) = new_game.make_move(&mv) {
                    let score = score_board(new_game.get_board(), self_colour);
                    move_options.push(MoveState {
                        move_: mv,
                        score,
                        game: new_game,
                    })
                } else {
                    // if the move is not valid, we skip it
                    continue;
                }
            }
        }
    }

    move_options.sort_by(|a, b| {
        // sort by score, descending
        b.score.cmp(&a.score)
    });

    move_options.pop().map(|m| (m.move_, m.score))
}

pub fn solve_next_move(game: &ChessGame) -> Result<Move, ChessError> {
    let self_colour = game.get_board().turn;
    let search_start_time = std::time::Instant::now();
    let context = RecursionContext {
        depth: 0,
        start_time: search_start_time,
    };
    let best_move = best_move_from_position(game, &self_colour, context);
    if let Some((mv, _score)) = best_move {
        Ok(mv)
    } else {
        Err(ChessError::SolverError("No valid moves found".to_string()))
    }
}
