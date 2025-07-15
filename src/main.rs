mod core;
mod engine;
mod input;
mod render;

use crate::core::Position;
use crate::engine::GameStatus;
use crate::input::parse_input_to_move;
use core::{ChessBoard, ChessColour, ChessError, ChessPiece};
use std::str::FromStr;

fn main() {
    let mut engine = engine::ChessEngine::default();

    render::display_board(engine.get_board());
    // loop for input
    let mut status = GameStatus::Ongoing;

    while status == GameStatus::Ongoing {
        let mut input = String::new();
        println!("Please enter the move you want to play:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let user_move = parse_input_to_move(&input);

        if let Ok(user_move) = user_move {
            match engine.make_move(user_move.from, user_move.to) {
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
