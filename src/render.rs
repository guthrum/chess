use crate::game::ChessGame;
use crate::{ChessBoard, ChessColour, ChessPiece};
use colored::Colorize;

pub fn display_board(game: &ChessGame, highlight_last_move: bool) {
    let chess_board = game.get_board();
    let last_move = game.played_moves().last();

    println!("    a b c d e f g h");
    println!("  ┌─────────────────┐");
    for (yidx, row) in chess_board.rows().rev().enumerate() {
        for (xidx, cell) in row.iter().enumerate() {
            if xidx == 0 {
                print!("{} │ ", 8 - yidx);
            }

            let mut c: char = cell.into();
            if c == '.' {
                c = '·'
            }
            let should_highlight = highlight_last_move
                && last_move.map_or(false, |m| {
                    (usize::from(m.from.column) == xidx && usize::from(m.from.row) == 7 - yidx)
                        || (usize::from(m.to.column) == xidx && usize::from(m.to.row) == 7 - yidx)
                });
            let colour = if should_highlight {
                c.to_string().red()
            } else if cell.colour == ChessColour::White {
                c.to_string().white()
            } else {
                c.to_string().blue()
            };

            print!("{colour} ");
            if xidx == 7 {
                println!("│");
            }
        }
    }
    println!("  └─────────────────┘");
    if chess_board.turn == ChessColour::White {
        println!("White's turn");
    }
}

fn render_piece(piece: &ChessPiece) -> char {
    piece.into()
}
