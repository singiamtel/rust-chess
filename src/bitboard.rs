#![allow(dead_code, unused)]
use crate::printer;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const MAX: Self = Self(0xFF_FF_FF_FF_FF_FF_FF_FF);
    pub const FROM_SQUARE: fn([u8; 2]) -> Self = |[file, rank]| Self(1 << (rank * 8 + file));

    pub fn north(self) -> Self {
        (self & NOT_RANK_8) << 8
    }

    pub fn south(self) -> Self {
        (self & NOT_RANK_1) >> 8
    }

    pub fn east(self) -> Self {
        (self & NOT_FILE_H) << 1
    }

    pub fn west(self) -> Self {
        (self & NOT_FILE_A) >> 1
    }

    pub fn north_east(self) -> Self {
        (self & (NOT_RANK_8 | NOT_FILE_H)) << 9
    }

    pub fn north_west(self) -> Self {
        (self & (NOT_RANK_8 | NOT_FILE_A)) << 7
    }

    pub fn south_east(self) -> Self {
        (self & (NOT_RANK_1 | NOT_FILE_H)) >> 7
    }

    pub fn south_west(self) -> Self {
        (self & (NOT_RANK_1 | NOT_FILE_A)) >> 9
    }

    pub fn pawn_initial(self, color_mask: Self) -> bool {
        self & PAWN_INITIAL & color_mask == self
    }

    pub fn move_bit(&mut self, from: Self, to: Self) {
        #[cfg(debug_assertions)]
        {
            assert_eq!(*self & from, from);
            assert_eq!(*self & to, Self(0));
        }
        *self ^= from;
        *self |= to;
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display: Vec<String> = printer::display_bitboard(*self);
        let formatted = display.iter().fold(String::new(), |mut acc, s| {
            writeln!(acc, "{s}").unwrap();
            acc
        });
        writeln!(f, "{formatted}")
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::Shl<u64> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self {
        Self(self.0 << rhs)
    }
}

impl std::ops::Shr<u64> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self {
        Self(self.0 >> rhs)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl std::ops::BitXor<usize> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self {
        Self(self.0 ^ rhs as u64)
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

pub const FILE_A: Bitboard = Bitboard(0x80_80_80_80_80_80_80_80);
pub const NOT_FILE_A: Bitboard = Bitboard(0x7f_7f_7f_7f_7f_7f_7f_7f);
pub const FILE_H: Bitboard = Bitboard(0x01_01_01_01_01_01_01_01);
pub const NOT_FILE_H: Bitboard = Bitboard(0xfe_fe_fe_fe_fe_fe_fe_fe);

// A-H
const FILES: [Bitboard; 8] = [
    Bitboard(0x01_01_01_01_01_01_01_01),
    Bitboard(0x02_02_02_02_02_02_02_02),
    Bitboard(0x04_04_04_04_04_04_04_04),
    Bitboard(0x08_08_08_08_08_08_08_08),
    Bitboard(0x10_10_10_10_10_10_10_10),
    Bitboard(0x20_20_20_20_20_20_20_20),
    Bitboard(0x40_40_40_40_40_40_40_40),
    Bitboard(0x80_80_80_80_80_80_80_80),
];

// 1-8
const RANKS: [Bitboard; 8] = [
    Bitboard(0x00_00_00_00_00_00_00_FF),
    Bitboard(0x00_00_00_00_00_00_FF_00),
    Bitboard(0x00_00_00_00_00_FF_00_00),
    Bitboard(0x00_00_00_00_FF_00_00_00),
    Bitboard(0x00_00_00_00_FF_00_00_00),
    Bitboard(0x00_00_00_FF_00_00_00_00),
    Bitboard(0x00_00_FF_00_00_00_00_00),
    Bitboard(0x00_FF_00_00_00_00_00_00),
];

pub const RANK_1: Bitboard = RANKS[0];
const NOT_RANK_1: Bitboard = Bitboard(0xFF_FF_FF_FF_FF_FF_FF_00);
pub const RANK_8: Bitboard = RANKS[7];
const NOT_RANK_8: Bitboard = Bitboard(0x00_FF_FF_FF_FF_FF_FF_FF);

const PAWN_INITIAL: Bitboard = Bitboard(0x00_FF_00_00_00_00_FF_00);
