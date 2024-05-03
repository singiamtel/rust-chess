use crate::{bitboard::Bitboard, piece::PieceKind, printer};

use std::fmt::Write;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Bitboard,
    pub to: Bitboard,
    pub capture: Option<PieceKind>, // To unmake move
    pub promotion: Option<PieceKind>,
    pub en_passant: Option<Bitboard>,
    pub castling: u8, // Keep track of changes to castling rights
}

impl Move {
    pub const fn new(from: Bitboard, to: Bitboard, promotion: Option<PieceKind>) -> Self {
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
            promotion,
            en_passant: None,
            castling: 0,
            capture: None,
        }
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
        write!(f, "{from} -> {to}")
    }
}
