use colored::{ColoredString, Colorize};
use crate::{ChessBoard, ChessColour, ChessPiece};

pub fn display_board(chess_board: &ChessBoard) {
    if chess_board.turn == ChessColour::Black {
        println!("Black's turn");
    }
    
    for (yidx, xidx, cell) in chess_board.row_iter() {
        let c: char = (&cell).into();
        let colour = if cell.colour == ChessColour::White {
            c.to_string().white()
        } else {
            c.to_string().blue()
        };

        print!("{} ", colour);
        if xidx == 7 {
            println!();
        }
    }
    if chess_board.turn == ChessColour::White {
        println!("White's turn");
    }
}

fn render_piece(piece: &ChessPiece) -> char {
    piece.into()
}