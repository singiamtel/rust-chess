use crate::{bitboard::BitboardError, r#move::Move};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovegenError {
    InvalidMove(String),
    BitboardError(BitboardError),
}

impl From<Move> for MovegenError {
    fn from(r#move: Move) -> Self {
        Self::InvalidMove(r#move.to_string())
    }
}

impl std::fmt::Display for MovegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidMove(r#move) => write!(f, "Invalid move: {}", r#move),
            Self::BitboardError(err) => write!(f, "Bitboard error: {}", err),
        }
    }
}

impl From<BitboardError> for MovegenError {
    fn from(err: BitboardError) -> Self {
        Self::BitboardError(err)
    }
}

impl Error for MovegenError {}
