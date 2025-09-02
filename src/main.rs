mod core;
mod game;
mod input;
mod render;
mod solver;
mod uci;

use crate::game::GameStatus;
use crate::input::parse_input_to_move;
use anyhow::Context;
use core::{ChessBoard, ChessColour, ChessError, ChessPiece};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(PartialEq)]
enum Mode {
    User,
    Stockfish,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_env("CHESS"))
        .init();

    let mut game = game::ChessGame::default();

    let mode = Mode::Stockfish;
    tracing::info!("Initializing game");
    let mut stockfish = uci::UciEngine::new("stockfish")?;
    tracing::info!("starting new game in stockfish");
    stockfish
        .new_game()
        .context("could not start new game in stockfish")?;
    tracing::info!("setting skill level to 1");
    stockfish
        .skill_level(1)
        .context("could not set skill level")?;
    stockfish.is_ready().context("could not isready")?;

    let mut status = GameStatus::Ongoing;
    let users_chess_colour = ChessColour::White;

    while status == GameStatus::Ongoing {
        render::display_board(&game, game.get_board().turn == users_chess_colour);
        if mode == Mode::User {
            while game.get_board().turn == users_chess_colour {
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
        } else {
            stockfish
                .position(&game.fen())
                .with_context(|| "could not set position in stockfish")?;
            // stockfish.is_ready().context("not ready after sending position")?;
            tracing::info!("waiting for stockfish to make a move");
            let best_move = stockfish
                .best_move()
                .expect("could not get best move from stockfish");
            tracing::info!("Best move: {best_move}");
            game.make_move(&best_move)
                .with_context(|| format!("could not make move: {best_move}"))?;
        }
        tracing::info!("waiting for solver to make a move");
        let mv = solver::solve_next_move(&game).with_context(|| "could not solve next move")?;
        let game_state = game.make_move(&mv)?;
        status = game_state.status;
        println!("Opponent played move: {} to {}", mv.from, mv.to);
    }
    Ok(())
}
