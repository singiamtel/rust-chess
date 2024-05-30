use crate::printer;
use std::{
    fmt::{Formatter, LowerHex, Result, Write},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr},
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
                0 => square.south_east() | square.south_west(),
                1 => square.north_east() | square.north_west(),
                _ => panic!("Invalid color"),
            };
            i += 1;
        }
        j += 1;
        i = 0;
    }
    lookup
}

pub fn generate_knight_lookup() -> [Bitboard; 64] {
    let mut lookup: [Bitboard; 64] = [Bitboard(0); 64];
    let mut i: u8 = 0;
    while i < 64 {
        let square = Bitboard(1 << i);
        // all 8 knight moves
        lookup[i as usize] = square.no_no_ea()
            | square.no_ea_ea()
            | square.so_ea_ea()
            | square.so_so_ea()
            | square.no_no_we()
            | square.no_we_we()
            | square.so_we_we()
            | square.so_so_we();
        i += 1;
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

impl Bitboard {
    pub const MAX: Self = Self(0xFF_FF_FF_FF_FF_FF_FF_FF);
    pub const fn from_square(file: u8, rank: u8) -> Self {
        Self(1 << (rank * 8 + file))
    }

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

    #[inline(always)]
    pub fn move_bit(&mut self, from: Self, to: Self) {
        #[cfg(debug_assertions)]
        {
            assert_eq!(*self & from, from);
        }
        *self ^= from;
        *self |= to;
    }

    #[inline(always)]
    pub fn clear_bit(&mut self, from: Self) {
        self.0 &= !from.0;
    }

    #[inline(always)]
    pub fn set_bit(&mut self, bit: Self) {
        *self |= bit;
    }

    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn intersects(self, other: Self) -> bool {
        (self & other) != Self(0)
    }

    #[inline(always)]
    pub fn idx(&self) -> usize {
        self.0.trailing_zeros() as usize
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl DirectionalShift for Bitboard {
    const NOT_FILE_A: Self = Self::NOT_FILE_A;
    const NOT_FILE_H: Self = Self::NOT_FILE_H;
    const NOT_FILE_AB: Self = Self::NOT_FILE_AB;
    const NOT_FILE_GH: Self = Self::NOT_FILE_GH;
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

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl Shl<u64> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self {
        Self(self.0 << rhs)
    }
}

impl Shr<u64> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl BitXor<usize> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self {
        Self(self.0 ^ rhs as u64)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl LowerHex for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let val = self.0;

        LowerHex::fmt(&val, f)
    }
}
