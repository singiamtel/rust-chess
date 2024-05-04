use crate::{
    bitboard::Bitboard,
    piece::{Piece, PieceKind},
    printer,
};

use std::fmt::Write;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub what: Piece,
    pub from: Bitboard,
    pub to: Bitboard,
    pub capture: Option<Piece>, // To unmake move
    pub promotion: Option<PieceKind>,
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
    const fn with_promotion(mut self, promotion: PieceKind) -> Self {
        self.promotion = Some(promotion);
        self
    }

    pub fn with_promotions(self) -> Vec<Self> {
        vec![
            self.with_promotion(PieceKind::Queen),
            self.with_promotion(PieceKind::Rook),
            self.with_promotion(PieceKind::Bishop),
            self.with_promotion(PieceKind::Knight),
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

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let from_display: Vec<String> = printer::display_bitboard(self.from);
        let to_display: Vec<String> = printer::display_bitboard(self.to);
        writeln!(f, "from:              to:")?;
        let format: fn(&str) -> String = |s: &str| -> String {
            s.chars().fold(String::new(), |mut output, c| -> String {
                write!(output, "{c} ").unwrap();
                output
            })
        };
        let formatted: String = from_display.iter().zip(to_display.iter()).fold(
            String::new(),
            |mut acc, (from, to)| -> String {
                if !acc.is_empty() {
                    writeln!(acc).unwrap();
                }
                write!(acc, "{} | {}", format(from), format(to)).unwrap();
                acc
            },
        );
        write!(f, "{formatted}").unwrap();
        Ok(())
    }
}

pub fn bitboard_to_algebraic(bitboard: Bitboard) -> String {
    assert!(
        bitboard.0.count_ones() == 1,
        "Bitboard is not a single square {bitboard}"
    );
    let file = (bitboard.0.trailing_zeros() % 8) as u8;
    let rank = (bitboard.0.trailing_zeros() / 8) as u8;
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
            ('a'..='h').contains(&file.chars().next().unwrap()),
            "File is not in range a-h: {algebraic} {bitboard}"
        );
        assert!(
            ('1'..='8').contains(&rank.chars().next().unwrap()),
            "Rank is not in range 1-8: {algebraic} {bitboard}"
        );
    }
    algebraic
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let from = bitboard_to_algebraic(self.from);
        let to = bitboard_to_algebraic(self.to);
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
            let en_passant = bitboard_to_algebraic(en_passant);
            let _ = write!(output, " e.p. {en_passant}");
        }
        if self.castling != 0 {
            let castling = self.castling;
            let _ = write!(output, " castling {castling}");
        }
        write!(f, "{output}")
    }
}
