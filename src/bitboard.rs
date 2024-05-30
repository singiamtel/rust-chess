use crate::printer;
use std::{
    fmt::Write,
    ops::{BitAnd, Shl, Shr},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub fn generate_pawn_lookup() -> [[Bitboard; 64]; 2] {
    let mut lookup: [[Bitboard; 64]; 2] = [[Bitboard(0); 64]; 2];
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    while j < 2 {
        while i < 64 {
            let square = Bitboard(1 << i);
            // 0 is white, 1 is black
            lookup[j as usize][i as usize] = match j {
                0 => square.north_east() | square.north_west(),
                1 => square.south_east() | square.south_west(),
                _ => panic!("Invalid color"),
            };
            i += 1;
        }
        j += 1;
        i = 0;
    }
    lookup
}

pub trait DirectionalShift:
    Sized + Shl<u64, Output = Self> + Shr<u64, Output = Self> + BitAnd<Self, Output = Self>
{
    const NOT_FILE_A: Self;
    const NOT_FILE_H: Self;
    const NOT_FILE_AB: Self;
    const NOT_FILE_GH: Self;

    fn north(self) -> Self {
        self << 8
    }

    fn south(self) -> Self {
        self >> 8
    }

    fn east(self) -> Self {
        (self & Self::NOT_FILE_H) << 1
    }

    fn west(self) -> Self {
        (self & Self::NOT_FILE_A) >> 1
    }

    fn north_east(self) -> Self {
        (self & Self::NOT_FILE_H) << 9
    }

    fn north_west(self) -> Self {
        (self & Self::NOT_FILE_A) << 7
    }

    fn south_east(self) -> Self {
        (self & Self::NOT_FILE_H) >> 7
    }

    fn south_west(self) -> Self {
        (self & Self::NOT_FILE_A) >> 9
    }
    // Knight moves
    fn no_no_ea(self) -> Self {
        (self & Self::NOT_FILE_H) << 17
    }
    fn no_ea_ea(self) -> Self {
        (self & Self::NOT_FILE_GH) << 10
    }
    fn so_ea_ea(self) -> Self {
        (self & Self::NOT_FILE_GH) >> 6
    }
    fn so_so_ea(self) -> Self {
        (self & Self::NOT_FILE_H) >> 15
    }
    fn no_no_we(self) -> Self {
        (self & Self::NOT_FILE_A) << 15
    }
    fn no_we_we(self) -> Self {
        (self & Self::NOT_FILE_AB) << 6
    }
    fn so_we_we(self) -> Self {
        (self & Self::NOT_FILE_AB) >> 10
    }
    fn so_so_we(self) -> Self {
        (self & Self::NOT_FILE_A) >> 17
    }
}

impl DirectionalShift for Bitboard {
    const NOT_FILE_A: Self = Bitboard(0xfe_fe_fe_fe_fe_fe_fe_fe);
    const NOT_FILE_H: Self = Bitboard(0x7f_7f_7f_7f_7f_7f_7f_7f);
    const NOT_FILE_AB: Self = Bitboard(0xfc_fc_fc_fc_fc_fc_fc_fc);
    const NOT_FILE_GH: Self = Bitboard(0x3f_3f_3f_3f_3f_3f_3f_3f);
}

impl DirectionalShift for u64 {
    const NOT_FILE_A: Self = 0xfe_fe_fe_fe_fe_fe_fe_fe;
    const NOT_FILE_H: Self = 0x7f_7f_7f_7f_7f_7f_7f_7f;
    const NOT_FILE_AB: Self = 0xfc_fc_fc_fc_fc_fc_fc_fc;
    const NOT_FILE_GH: Self = 0x3f_3f_3f_3f_3f_3f_3f_3f;
}

impl Bitboard {
    pub const MAX: Self = Self(0xFF_FF_FF_FF_FF_FF_FF_FF);
    pub const fn from_square(file: u8, rank: u8) -> Self {
        Self(1 << (rank * 8 + file))
    }
    // const fn from_square(square: [u8; 2]) -> Bitboard {
    //     Bitboard(1 << (square[1] * 8 + square[0]))
    // }

    // A-H
    const FILES: [Self; 8] = [
        Self(0x01_01_01_01_01_01_01_01),
        Self(0x02_02_02_02_02_02_02_02),
        Self(0x04_04_04_04_04_04_04_04),
        Self(0x08_08_08_08_08_08_08_08),
        Self(0x10_10_10_10_10_10_10_10),
        Self(0x20_20_20_20_20_20_20_20),
        Self(0x40_40_40_40_40_40_40_40),
        Self(0x80_80_80_80_80_80_80_80),
    ];

    // 1-8
    const RANKS: [Self; 8] = [
        Self(0x00_00_00_00_00_00_00_FF),
        Self(0x00_00_00_00_00_00_FF_00),
        Self(0x00_00_00_00_00_FF_00_00),
        Self(0x00_00_00_00_FF_00_00_00),
        Self(0x00_00_00_00_FF_00_00_00),
        Self(0x00_00_00_FF_00_00_00_00),
        Self(0x00_00_FF_00_00_00_00_00),
        Self(0x00_FF_00_00_00_00_00_00),
    ];

    pub const FILE_H: Self = Self::FILES[7];
    pub const NOT_FILE_H: Self = Self(0x7f_7f_7f_7f_7f_7f_7f_7f);
    pub const FILE_A: Self = Self::FILES[0];
    pub const NOT_FILE_A: Self = Self(0xfe_fe_fe_fe_fe_fe_fe_fe);
    pub const FILE_GH: Self = Self(0xC0_C0_C0_C0_C0_C0_C0_C0);
    pub const NOT_FILE_GH: Self = Self(0x3f_3f_3f_3f_3f_3f_3f_3f);
    pub const FILE_AB: Self = Self(0x03_03_03_03_03_03_03_03);
    pub const NOT_FILE_AB: Self = Self(0xfc_fc_fc_fc_fc_fc_fc_fc);

    pub const RANK_1: Self = Self::RANKS[0];
    pub const RANK_8: Self = Self::RANKS[7];

    const PAWN_INITIAL: Self = Self(0x00_FF_00_00_00_00_FF_00);

    pub fn pawn_initial(self, color_mask: Self) -> bool {
        (self & Self::PAWN_INITIAL & color_mask) == self
    }

    pub fn move_bit(&mut self, from: Self, to: Self) {
        #[cfg(debug_assertions)]
        {
            assert_eq!(*self & from, from);
        }
        *self ^= from;
        *self |= to;
    }

    pub fn clear_bit(&mut self, from: Self) {
        self.0 &= !from.0;
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn intersects(self, other: Self) -> bool {
        (self & other) != Self(0)
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display: Vec<String> = printer::display_bitboard(*self);
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
