use crate::{
    bitboard::{display::BitboardDisplay, Bitboard},
    piece::{Kind, Piece},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.from
                .to_algebraic()
                .unwrap_or_else(|_| "EE".to_string()),
            self.to.to_algebraic().unwrap_or_else(|_| "EE".to_string())
        )
    }
}
