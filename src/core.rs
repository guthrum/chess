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
            ChessError::InvalidPiece(msg) => write!(f, "Invalid chess piece: {msg}"),
            ChessError::InvalidMove(msg) => write!(f, "Invalid move: {msg}"),
        }
    }
}

impl Error for ChessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
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

impl Row {
    pub fn try_add(&self, amount: isize) -> Result<Self, ChessError> {
        let u: usize = (*self).into();
        Self::try_from((u as isize) + amount)
    }
}

impl FromStr for Row {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(Row::One),
            "2" => Ok(Row::Two),
            "3" => Ok(Row::Three),
            "4" => Ok(Row::Four),
            "5" => Ok(Row::Five),
            "6" => Ok(Row::Six),
            "7" => Ok(Row::Seven),
            "8" => Ok(Row::Eight),
            _ => Err(ChessError::InvalidMove(format!("Invalid row: '{s}'"))),
        }
    }
}

impl From<Row> for usize {
    fn from(val: Row) -> Self {
        match val {
            Row::One => 0,
            Row::Two => 1,
            Row::Three => 2,
            Row::Four => 3,
            Row::Five => 4,
            Row::Six => 5,
            Row::Seven => 6,
            Row::Eight => 7,
        }
    }
}

impl TryFrom<isize> for Row {
    type Error = ChessError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Row::One),
            1 => Ok(Row::Two),
            2 => Ok(Row::Three),
            3 => Ok(Row::Four),
            4 => Ok(Row::Five),
            5 => Ok(Row::Six),
            6 => Ok(Row::Seven),
            7 => Ok(Row::Eight),
            _ => Err(ChessError::InvalidMove(format!("Invalid row: '{value}'"))),
        }
    }
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

impl Column {
    pub fn try_add(&self, amount: isize) -> Result<Self, ChessError> {
        let u: usize = (*self).into();
        Self::try_from((u as isize) + amount)
    }
}

impl FromStr for Column {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "a" => Ok(Column::A),
            "b" => Ok(Column::B),
            "c" => Ok(Column::C),
            "d" => Ok(Column::D),
            "e" => Ok(Column::E),
            "f" => Ok(Column::F),
            "g" => Ok(Column::G),
            "h" => Ok(Column::H),
            _ => Err(ChessError::InvalidMove(format!("Invalid column: '{s}'"))),
        }
    }
}

impl TryFrom<isize> for Column {
    type Error = ChessError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Column::A),
            1 => Ok(Column::B),
            2 => Ok(Column::C),
            3 => Ok(Column::D),
            4 => Ok(Column::E),
            5 => Ok(Column::F),
            6 => Ok(Column::G),
            7 => Ok(Column::H),
            _ => Err(ChessError::InvalidMove(format!(
                "Invalid column: '{value}'"
            ))),
        }
    }
}

impl From<Column> for usize {
    fn from(val: Column) -> Self {
        match val {
            Column::A => 0,
            Column::B => 1,
            Column::C => 2,
            Column::D => 3,
            Column::E => 4,
            Column::F => 5,
            Column::G => 6,
            Column::H => 7,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    pub row: Row,
    pub column: Column,
}

impl Position {
    pub fn board_position(&self) -> (usize, usize) {
        (self.column.into(), self.row.into())
    }

    pub fn add_offset(&self, row: isize, column: isize) -> Result<Self, ChessError> {
        Ok(Self {
            row: self.row.try_add(row)?,
            column: self.column.try_add(column)?,
        })
    }
}

impl FromStr for Position {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse the format a1, c6, ...
        if s.len() != 2 {
            return Err(ChessError::InvalidMove(format!(
                "Invalid position format: '{s}'"
            )));
        }
        let column = Column::from_str(s.get(0..1).ok_or_else(|| {
            ChessError::InvalidMove(format!("Invalid column in position: '{s}'"))
        })?)?;
        let row = Row::from_str(s.get(1..2).ok_or_else(|| {
            ChessError::InvalidMove(format!("Invalid row in position: '{s}'"))
        })?)?;
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

impl ChessColour {
    pub fn direction(&self) -> isize {
        match self {
            ChessColour::White => 1,  // White moves up the board
            ChessColour::Black => -1, // Black moves down the board
        }
    }

    pub fn flip(&self) -> ChessColour {
        match self {
            ChessColour::White => ChessColour::Black,
            ChessColour::Black => ChessColour::White,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct ChessPiece {
    pub kind: ChessPieceKind,
    pub colour: ChessColour,
    pub moved: bool,
}

impl From<&ChessPiece> for char {
    fn from(val: &ChessPiece) -> Self {
        let c = match val.kind {
            ChessPieceKind::Pawn => 'p',
            ChessPieceKind::Knight => 'n',
            ChessPieceKind::Bishop => 'b',
            ChessPieceKind::Rook => 'r',
            ChessPieceKind::Queen => 'q',
            ChessPieceKind::King => 'k',
        };
        if val.colour == ChessColour::White {
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
            _ => Err(ChessError::InvalidPiece(format!(
                "Invalid chess piece character: '{c}'"
            ))),
        }?;
        let colour = if c.is_uppercase() {
            ChessColour::White
        } else {
            ChessColour::Black
        };
        Ok(ChessPiece {
            kind,
            colour,
            moved: false,
        })
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

impl From<&Cell> for char {
    fn from(val: &Cell) -> Self {
        match &val.piece {
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
    pub(crate) fn get_piece_at(&self, pos: &Position) -> Option<&Cell> {
        let x: usize = pos.row.into();
        let y: usize = pos.column.into();
        if x < 8 && y < 8 {
            Some(&self.board[x][y])
        } else {
            None
        }
    }
}

impl ChessBoard {
    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[Cell; 8]> {
        self.board.iter()
    }
}

impl FromStr for ChessBoard {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = [[Cell {
            piece: None,
            colour: ChessColour::White,
        }; 8]; 8];
        for (i, line) in s
            .lines()
            .rev()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .enumerate()
        {
            if i >= 8 {
                return Err(ChessError::InvalidPiece(
                    "too many rows on chess board".to_string(),
                ));
            }
            for (j, c) in line.chars().enumerate() {
                if j >= 8 {
                    return Err(ChessError::InvalidPiece(
                        "too many columns on chess board".to_string(),
                    ));
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
        .pp..ppp
        ........
        p..pp...
        P..PP...
        ........
        .PP..PPP
        RNBQKBRN
    "#;
        let board = ChessBoard::from_str(board_str).expect("Failed to parse chess board");
        ChessBoard {
            board: board.board,
            turn: ChessColour::White,
        }
    }
}
