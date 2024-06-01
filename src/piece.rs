use std::ops::Not;

use crate::bitboard::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
    pub position: Bitboard,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.kind)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Piece {
    #[must_use]
    pub const fn new(color: Color, kind: Kind, position: Bitboard) -> Self {
        Self {
            color,
            kind,
            position,
        }
    }
}

#[must_use]
pub fn to_letter(piece: Option<Piece>) -> char {
    let mut c: char = piece.map_or('.', |piece| match piece.kind {
        Kind::Pawn => 'P',
        Kind::Knight => 'N',
        Kind::Bishop => 'B',
        Kind::Rook => 'R',
        Kind::Queen => 'Q',
        Kind::King => 'K',
    });
    if let Some(piece) = piece {
        c = match piece.color {
            Color::White => c,
            Color::Black => c.to_ascii_lowercase(),
        };
    }
    c
}
