use crate::core::{ChessError, Column, Move, Position, Row};
use std::str::FromStr;

pub fn parse_input_to_move(input: &str) -> Result<Move, ChessError> {
    let input = input.trim();
    let (first, second) = if input.contains(' ') {
        let mut parts = input.split_whitespace();
        (
            parts.next().ok_or(ChessError::InvalidMove(
                "No 'from' position provided".to_string(),
            ))?,
            parts.next().ok_or(ChessError::InvalidMove(
                "No 'to' position provided".to_string(),
            ))?,
        )
    } else {
        if input.len() < 3 {
            return Err(ChessError::InvalidMove(
                "Input must be at least 3 characters long".to_string(),
            ));
        }
        (&input[0..2], &input[2..])
    };

    let from = Position::from_str(first)
        .map_err(|_| ChessError::InvalidMove(format!("Invalid 'from' position: '{}'", first)))?;
    let to = if second.len() == 1 {
        let c = second.chars().next().unwrap();
        if c.is_digit(10) {
            Position {
                row: Row::from_str(second)?,
                column: from.column,
            }
        } else {
            Position {
                row: from.row,
                column: Column::from_str(second)?,
            }
        }
    } else {
        Position::from_str(second)
            .map_err(|_| ChessError::InvalidMove(format!("Invalid 'to' position: '{}'", first)))?
    };

    Ok(Move { from, to })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::{Column, Row};

    #[test]
    fn test_parse_simple_inputs() {
        let expected = Move {
            from: Position {
                row: Row::Two,
                column: Column::A,
            },
            to: Position {
                row: Row::Three,
                column: Column::A,
            },
        };
        for input in ["a2 a3", "a2a3", " a2    a3", "a23"] {
            let move_result = parse_input_to_move(input);
            assert!(move_result.is_ok(), "Failed to parse input: {}", input);
            let chess_move = move_result.unwrap();
            assert_eq!(
                chess_move, expected,
                "Parsed move does not match expected for input: {}",
                input
            );
        }
    }
}
