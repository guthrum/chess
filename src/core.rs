use std::error::Error;
use std::str::FromStr;


#[derive(Debug)]
pub enum ChessError {
    InvalidPiece(String),
    InvalidMove(String),
}

impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessError::InvalidPiece(msg) => write!(f, "Invalid chess piece: {}", msg),
            ChessError::InvalidMove(msg) => write!(f, "Invalid move: {}", msg),
        }
    }
}

impl Error for ChessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Row {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    pub row: Row,
    pub column: Column,
}

impl FromStr for Position {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse the format a1, c6, ...
        if s.len() != 2 {
            return Err(ChessError::InvalidMove(format!("Invalid position format: '{}'", s)));
        }
        let mut chars = s.chars();
        let column = match chars.next().map(|s| s.to_ascii_lowercase()) {
            Some('a') => Column::A,
            Some('b') => Column::B,
            Some('c') => Column::C,
            Some('d') => Column::D,
            Some('e') => Column::E,
            Some('f') => Column::F,
            Some('g') => Column::G,
            Some('h') => Column::H,
            _ => return Err(ChessError::InvalidMove(format!("Invalid column in position: '{}'", s))),
        };
        let row = match chars.next() {
            Some('1') => Row::One,
            Some('2') => Row::Two,
            Some('3') => Row::Three,
            Some('4') => Row::Four,
            Some('5') => Row::Five,
            Some('6') => Row::Six,
            Some('7') => Row::Seven,
            Some('8') => Row::Eight,
            _ => return Err(ChessError::InvalidMove(format!("Invalid row in position: '{}'", s))),
        };
        Ok(Position { row, column })
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}



#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ChessPieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ChessColour {
    White,
    Black,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct ChessPiece {
    pub kind: ChessPieceKind,
    pub colour: ChessColour,
}

impl Into<char> for &ChessPiece {
    fn into(self) -> char {
        let c = match self.kind {
            ChessPieceKind::Pawn => 'p',
            ChessPieceKind::Knight => 'n',
            ChessPieceKind::Bishop => 'b',
            ChessPieceKind::Rook => 'r',
            ChessPieceKind::Queen => 'q',
            ChessPieceKind::King => 'k',
        };
        if self.colour == ChessColour::White {
            c.to_ascii_uppercase()
        } else {
            c.to_ascii_lowercase()
        }
    }
}

impl TryFrom<char> for ChessPiece {
    type Error = ChessError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let kind = match c.to_ascii_lowercase() {
            'p' => Ok(ChessPieceKind::Pawn),
            'n' => Ok(ChessPieceKind::Knight),
            'b' => Ok(ChessPieceKind::Bishop),
            'r' => Ok(ChessPieceKind::Rook),
            'q' => Ok(ChessPieceKind::Queen),
            'k' => Ok(ChessPieceKind::King),
            _ => Err(ChessError::InvalidPiece(format!("Invalid chess piece character: '{}'", c))),
        }?;
        let colour = if c.is_uppercase() {
            ChessColour::White
        } else {
            ChessColour::Black
        };
        Ok(ChessPiece { kind, colour })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Cell {
    pub piece: Option<ChessPiece>,
    pub colour: ChessColour,
}

impl Cell {
    pub fn parse(value: char, pos: (usize, usize)) -> Result<Self, ChessError> {
        let piece = if value == '.' {
            None
        } else {
            Some(ChessPiece::try_from(value)?)
        };
        let colour = if (pos.1 + pos.0) % 2 == 0 {
            ChessColour::White
        } else {
            ChessColour::Black
        };
        Ok(Cell { piece, colour })
    }
}

impl Into<char> for &Cell {
    fn into(self) -> char {
        match &self.piece {
            Some(piece) => piece.into(),
            None => '.',
        }
    }
}

pub struct ChessBoard {
    pub board: [[Cell; 8]; 8],
    pub turn: ChessColour,
}

impl ChessBoard {
    pub fn row_iter(&self) -> impl Iterator<Item = (usize, usize, Cell)> {
        self.board.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, piece)| (i, j, *piece))
        })
    }
}

impl FromStr for ChessBoard {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = [[Cell {
            piece: None,
            colour: ChessColour::White,
        }; 8]; 8];
        for (i, line) in s.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .enumerate() {
            if i >= 8 {
                return Err(ChessError::InvalidPiece("too many rows on chess board".to_string()));
            }
            for (j, c) in line.chars().enumerate() {
                if j >= 8 {
                    return Err(ChessError::InvalidPiece("too many columns on chess board".to_string()));
                }
                board[i][j] = Cell::parse(c, (i, j))?;
            }
        }

        Ok(Self {
            board,
            turn: ChessColour::White,
        })
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        let board_str = r#"
        rnbqkbrn
        pppppppp
        ........
        ........
        ........
        ........
        PPPPPPPP
        RNBQKBRN
    "#;
        let board = ChessBoard::from_str(board_str)
            .expect("Failed to parse chess board");
        ChessBoard {
            board: board.board,
            turn: ChessColour::White,
        }
    }
}

