use crate::{
    bitboard::Bitboard,
    piece::{Kind, Piece},
};

use std::fmt::Write;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub what: Piece,
    pub from: Bitboard,
    pub to: Bitboard,
    pub capture: Option<Piece>, // To unmake move
    pub promotion: Option<Kind>,
    pub en_passant: Option<Bitboard>,
    pub castling: u8, // Keep track of changes to castling rights
}

impl Move {
    pub const fn new(from: Bitboard, to: Bitboard, what: Piece) -> Self {
        // #[cfg(debug_assertions)]
        // {
        //     assert!(
        //         !(from & to != Bitboard(0)),
        //         "From and to squares are the same"
        //     );
        //     assert!(
        //         !(piece.kind != PieceKind::Pawn && promotion.is_some()),
        //         "Non-pawn piece cannot promote"
        //     );
        //     assert!(
        //         !(to & (RANK_1 | RANK_8) == Bitboard(0) && promotion.is_some()),
        //         "Pawn must promote on rank 1 or 8"
        //     );
        // }
        Self {
            from,
            to,
            what,
            promotion: None,
            en_passant: None,
            castling: 0,
            capture: None,
        }
    }
    const fn with_promotion(mut self, promotion: Kind) -> Self {
        self.promotion = Some(promotion);
        self
    }

    pub fn with_promotions(self) -> Vec<Self> {
        vec![
            self.with_promotion(Kind::Queen),
            self.with_promotion(Kind::Rook),
            self.with_promotion(Kind::Bishop),
            self.with_promotion(Kind::Knight),
        ]
    }
    // pub const fn with_en_passant(mut self, en_passant: Bitboard) -> Self {
    //     self.en_passant = Some(en_passant);
    //     self
    // }
    // pub const fn with_castling(mut self, castling: u8) -> Self {
    //     self.castling = castling;
    //     self
    // }
    pub const fn with_capture(mut self, capture: Piece) -> Self {
        self.capture = Some(capture);
        self
    }
}
use std::fmt::Display;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum BitboardError {
    InvalidSingleSquare(String),
    NoPieceAtSquare(Bitboard),
    TryFromIntError(TryFromIntError),
}

impl Display for BitboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidSingleSquare(s) => write!(f, "Invalid single square: {s}"),
            Self::TryFromIntError(e) => write!(f, "TryFromIntError: {e}"),
            Self::NoPieceAtSquare(b) => write!(f, "No piece at square: {b}"),
        }
    }
}

impl From<TryFromIntError> for BitboardError {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromIntError(e)
    }
}

impl std::error::Error for BitboardError {}

pub fn bitboard_to_algebraic(bitboard: Bitboard) -> Result<String, BitboardError> {
    if !bitboard.0.count_ones() == 1 {
        return Err(BitboardError::InvalidSingleSquare(bitboard.0.to_string()));
    }
    let file = u8::try_from(bitboard.0.trailing_zeros() % 8)?;
    let rank = u8::try_from(bitboard.0.trailing_zeros() / 8)?;
    let mut algebraic = String::new();
    let _ = write!(algebraic, "{}{}", (file + b'a') as char, rank + 1);
    #[cfg(debug_assertions)]
    {
        assert!(
            algebraic.len() == 2,
            "Algebraic notation is not 2 characters long"
        );
        let (file, rank) = algebraic.split_at(1);
        assert!(
            ('a'..='h').contains(
                &file
                    .chars()
                    .next()
                    .ok_or_else(|| BitboardError::InvalidSingleSquare(algebraic.clone()))?
            ),
            "File is not in range a-h: {algebraic} {bitboard}"
        );
        assert!(
            ('1'..='8').contains(
                &rank
                    .chars()
                    .next()
                    .ok_or_else(|| BitboardError::InvalidSingleSquare(algebraic.clone()))?
            ),
            "Rank is not in range 1-8: {algebraic} {bitboard}"
        );
    }
    Ok(algebraic)
}

pub fn algebraic_to_bitboard(algebraic: &str) -> Result<Bitboard, BitboardError> {
    if algebraic.len() != 2 {
        return Err(BitboardError::InvalidSingleSquare(algebraic.to_string()));
    }
    let mut chars = algebraic.chars();
    let file = chars
        .next()
        .ok_or_else(|| BitboardError::InvalidSingleSquare(algebraic.to_string()))?;
    let rank = chars
        .next()
        .ok_or_else(|| BitboardError::InvalidSingleSquare(algebraic.to_string()))?;
    let file = file as u8 - b'a';
    let rank = rank as u8 - b'1';
    let bitboard = Bitboard(1 << (rank * 8 + file));
    #[cfg(debug_assertions)]
    {
        assert!(
            bitboard.0.count_ones() == 1,
            "Bitboard is not a single square: {algebraic} {bitboard}"
        );
    }
    Ok(bitboard)
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let from: String = bitboard_to_algebraic(self.from).unwrap_or_else(|_| "EE".to_string());
        let to: String = bitboard_to_algebraic(self.to).unwrap_or_else(|_| "EE".to_string());
        let what = self.what;
        let mut output = String::new();
        let _ = write!(output, "{what} {from} -> {to}");
        if let Some(promotion) = self.promotion {
            let promotion = Piece::new(what.color, promotion);
            let _ = write!(output, " {promotion}");
        }
        if let Some(capture) = self.capture {
            let _ = write!(output, " x {capture}");
        }
        if let Some(en_passant) = self.en_passant {
            let en_passant = bitboard_to_algebraic(en_passant).unwrap_or_else(|_| "EE".to_string());
            let _ = write!(output, " e.p. {en_passant}");
        }
        if self.castling != 0 {
            let castling = self.castling;
            let _ = write!(output, " castling {castling}");
        }
        write!(f, "{output}")
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            bitboard_to_algebraic(self.from).unwrap_or_else(|_| "EE".to_string()),
            bitboard_to_algebraic(self.to).unwrap_or_else(|_| "EE".to_string())
        )
    }
}
