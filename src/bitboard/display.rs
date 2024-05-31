// There are 2 main formats supported by a bitboard:
// 1. Bitboard: A 64-bit integer representing the board. (e.g. 0x80000000000000000)
// 2. Algebraic: A string representing the board in algebraic notation. (e.g. e4)

use crate::bitboard::{Bitboard, BitboardError};

use std::fmt::Write;

pub trait BitboardDisplay: Sized {
    fn from_algebraic(algebraic: &str) -> Result<Self, BitboardError>;
    fn to_algebraic(&self) -> Result<String, BitboardError>;
    fn display_bitboard(&self) -> Vec<String>;
}

impl BitboardDisplay for Bitboard {
    fn from_algebraic(algebraic: &str) -> Result<Self, BitboardError> {
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

    fn to_algebraic(&self) -> Result<String, BitboardError> {
        if !self.0.count_ones() == 1 {
            return Err(BitboardError::InvalidSingleSquare(self.0.to_string()));
        }
        let file = u8::try_from(self.0.trailing_zeros() % 8)?;
        let rank = u8::try_from(self.0.trailing_zeros() / 8)?;
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
                "File is not in range a-h: {algebraic} {self}"
            );
            assert!(
                ('1'..='8').contains(
                    &rank
                        .chars()
                        .next()
                        .ok_or_else(|| BitboardError::InvalidSingleSquare(algebraic.clone()))?
                ),
                "Rank is not in range 1-8: {algebraic} {self}"
            );
        }
        Ok(algebraic)
    }
    fn display_bitboard(&self) -> Vec<String> {
        let mut board = [['.'; 8]; 8];
        for i in 0..64 {
            if (self.0 & (1 << i)) != 0 {
                let row = i / 8;
                let col = i % 8;
                board[row][col] = 'X';
            }
        }
        board
            .iter()
            .rev()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display: Vec<String> = self.display_bitboard();
        writeln!(f)?;
        let formatted = display.iter().fold(String::new(), |mut acc, s| {
            s.chars().fold(&mut acc, |acc, c| {
                write!(acc, "{c} ").unwrap();
                acc
            });
            writeln!(acc).unwrap();
            acc
        });
        writeln!(f, "{formatted}")
    }
}
