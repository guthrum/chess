use crate::{ChessBoard, ChessColour, ChessPiece};
use colored::Colorize;

pub fn display_board(chess_board: &ChessBoard) {
    if chess_board.turn == ChessColour::Black {
        println!("Black's turn");
    }

    println!("    a b c d e f g h");
    println!("  ┌─────────────────┐");
    for (yidx, xidx, cell) in chess_board.row_iter() {
        if xidx == 0 {
            print!("{} │ ", 8 - yidx);
        }

        let mut c: char = (&cell).into();
        if c == '.' {
            c = '·'
        }
        let colour = if cell.colour == ChessColour::White {
            c.to_string().white()
        } else {
            c.to_string().blue()
        };

        print!("{} ", colour);
        if xidx == 7 {
            println!("│");
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
