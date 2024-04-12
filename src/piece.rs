pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

pub enum Color {
    White,
    Black,
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
