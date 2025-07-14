mod engine;
mod render;
mod core;

use std::str::FromStr;
use core::{ChessError, ChessBoard, ChessColour, ChessPiece};
use crate::core::Position;
use crate::engine::GameStatus;

fn main() {
    let mut engine = engine::ChessEngine::default();
    
    render::display_board(engine.get_board());
    // loop for input
    let mut status = GameStatus::Ongoing;
    
    while status == GameStatus::Ongoing {
        let mut input = String::new();
        println!("Please enter the move you want to play:");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let mut parts = input.split_whitespace();

        let from = parts.next()
            .ok_or(ChessError::InvalidMove("No 'from' position provided".to_string()))
            .and_then(|v| Position::from_str(v));
        let to = parts.next()
            .ok_or(ChessError::InvalidMove("No 'to' position provided".to_string()))
            .and_then(|v| Position::from_str(v));

        if let (Ok(from), Ok(to)) = (from, to) {

            match engine.make_move(from, to) {
                Ok(game_state) => {
                    status = game_state.status;
                    render::display_board(engine.get_board());
                }
                Err(e) => {
                    println!("Error making move: {}", e);
                }
            }
        } else {
            println!("Invalid input. Please enter moves in the format 'from to' (e.g., 'e2 e4').");
        }
    }
}
