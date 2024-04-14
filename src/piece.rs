#![allow(dead_code, unused)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.color, self.kind)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl std::fmt::Display for PieceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const PIECE_CHARS: [(PieceKind, char); 12] = [
    (PieceKind::Pawn, 'P'),
    (PieceKind::Knight, 'N'),
    (PieceKind::Bishop, 'B'),
    (PieceKind::Rook, 'R'),
    (PieceKind::Queen, 'Q'),
    (PieceKind::King, 'K'),
    (PieceKind::Pawn, 'p'),
    (PieceKind::Knight, 'n'),
    (PieceKind::Bishop, 'b'),
    (PieceKind::Rook, 'r'),
    (PieceKind::Queen, 'q'),
    (PieceKind::King, 'k'),
];

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }
}
