mod core;
mod game;
mod input;
mod render;
mod solver;

use crate::game::GameStatus;
use crate::input::parse_input_to_move;
use core::{ChessBoard, ChessColour, ChessError, ChessPiece};

fn main() {
    let mut game = game::ChessGame::default();

    let mut status = GameStatus::Ongoing;

    while status == GameStatus::Ongoing {
        render::display_board(game.get_board());
        while game.get_board().turn == ChessColour::White {
            let mut input = String::new();
            println!("Please enter the move you want to play:");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let user_move = parse_input_to_move(&input);

            if let Ok(user_move) = user_move {
                match game.make_move(&user_move) {
                    Ok(game_state) => {
                        status = game_state.status;
                    }
                    Err(e) => {
                        println!("Error making move: {e}");
                    }
                }
            } else {
                println!(
                    "Invalid input. Please enter moves in the format 'from to' (e.g., 'e2 e4')."
                );
            }
        }
        let mv = solver::solve_next_move(&game).expect("solver failed to find a move");
        match game.make_move(&mv) {
            Ok(game_state) => {
                status = game_state.status;
                println!("Opponent played move: {} to {}", mv.from, mv.to);
            }
            Err(e) => {
                println!("Error making move: {e}");
                return;
            }
        }
    }
}
