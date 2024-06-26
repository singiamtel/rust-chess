use std::{
    fmt::{Display, Formatter, LowerHex},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    NoNoEa,
    NoEaEa,
    SoEaEa,
    SoSoEa,
    NoNoWe,
    NoWeWe,
    SoWeWe,
    SoSoWe,
}

macro_rules! define_moves {
    ($name:ident, $start:expr, $end:expr) => {
        pub const $name: [Self; $end - $start] = {
            let mut moves = [Self::North; $end - $start];
            let mut i = 0;
            while i < $end - $start {
                moves[i] = Self::SLIDING_MOVES[$start + i];
                i += 1;
            }
            moves
        };
    };
}

impl Direction {
    pub const SLIDING_MOVES: [Self; 8] = [
        Self::North,
        Self::South,
        Self::East,
        Self::West,
        Self::NorthEast,
        Self::NorthWest,
        Self::SouthEast,
        Self::SouthWest,
    ];

    define_moves!(STRAIGHT_MOVES, 0, 4);
    define_moves!(DIAGONAL_MOVES, 4, 8);

    pub const KNIGHT_MOVES: [Self; 8] = [
        Self::NoNoEa,
        Self::NoEaEa,
        Self::SoEaEa,
        Self::SoSoEa,
        Self::NoNoWe,
        Self::NoWeWe,
        Self::SoWeWe,
        Self::SoSoWe,
    ];
    pub const fn pawn_captures(color: Color) -> [Self; 2] {
        match color {
            Color::White => [Self::NorthEast, Self::NorthWest],
            Color::Black => [Self::SouthEast, Self::SouthWest],
        }
    }
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

    #[inline(always)]
    fn north(self) -> Self {
        self << 8
    }

    #[inline(always)]
    fn south(self) -> Self {
        self >> 8
    }

    #[inline(always)]
    fn east(self) -> Self {
        (self & Self::NOT_FILE_H) << 1
    }

    #[inline(always)]
    fn west(self) -> Self {
        (self & Self::NOT_FILE_A) >> 1
    }

    #[inline(always)]
    fn north_east(self) -> Self {
        (self & Self::NOT_FILE_H) << 9
    }

    #[inline(always)]
    fn north_west(self) -> Self {
        (self & Self::NOT_FILE_A) << 7
    }

    #[inline(always)]
    fn south_east(self) -> Self {
        (self & Self::NOT_FILE_H) >> 7
    }

    #[inline(always)]
    fn south_west(self) -> Self {
        (self & Self::NOT_FILE_A) >> 9
    }
    // Knight moves
    #[inline(always)]
    fn no_no_ea(self) -> Self {
        (self & Self::NOT_FILE_H) << 17
    }
    #[inline(always)]
    fn no_ea_ea(self) -> Self {
        (self & Self::NOT_FILE_GH) << 10
    }
    #[inline(always)]
    fn so_ea_ea(self) -> Self {
        (self & Self::NOT_FILE_GH) >> 6
    }
    #[inline(always)]
    fn so_so_ea(self) -> Self {
        (self & Self::NOT_FILE_H) >> 15
    }
    #[inline(always)]
    fn no_no_we(self) -> Self {
        (self & Self::NOT_FILE_A) << 15
    }
    #[inline(always)]
    fn no_we_we(self) -> Self {
        (self & Self::NOT_FILE_AB) << 6
    }
    #[inline(always)]
    fn so_we_we(self) -> Self {
        (self & Self::NOT_FILE_AB) >> 10
    }
    #[inline(always)]
    fn so_so_we(self) -> Self {
        (self & Self::NOT_FILE_A) >> 17
    }
    fn shift(self, direction: Direction) -> Self {
        match direction {
            Direction::North => self.north(),
            Direction::South => self.south(),
            Direction::East => self.east(),
            Direction::West => self.west(),
            Direction::NorthEast => self.north_east(),
            Direction::NorthWest => self.north_west(),
            Direction::SouthEast => self.south_east(),
            Direction::SouthWest => self.south_west(),
            Direction::NoNoEa => self.no_no_ea(),
            Direction::NoEaEa => self.no_ea_ea(),
            Direction::SoEaEa => self.so_ea_ea(),
            Direction::SoSoEa => self.so_so_ea(),
            Direction::NoNoWe => self.no_no_we(),
            Direction::NoWeWe => self.no_we_we(),
            Direction::SoWeWe => self.so_we_we(),
            Direction::SoSoWe => self.so_so_we(),
        }
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
        Self(0x00_00_00_FF_00_00_00_00),
        Self(0x00_00_FF_00_00_00_00_00),
        Self(0x00_FF_00_00_00_00_00_00),
        Self(0xFF_00_00_00_00_00_00_00),
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
    pub const PAWN_PROMOTION_MASK: Self = Bitboard(Self::RANK_8.0 | Self::RANK_1.0);

    const PAWN_INITIAL: Self = Self(0x00_FF_00_00_00_00_FF_00);
    // Some day :(
    // pub const KING_INITIAL: Self = Self::from_algebraic("e1").unwrap() & Self::from_algebraic("e8").unwrap();
    pub const KING_INITIAL: Self = Self(0x10_00_00_00_00_00_00_10);

    pub fn pawn_initial(self, color_mask: Self) -> bool {
        (self & Self::PAWN_INITIAL & color_mask) == self
    }

    #[inline(always)]
    pub fn move_bit(&mut self, from: Self, to: Self) {
        #[cfg(debug_assertions)]
        {
            assert_eq!(from.count(), 1);
            assert_eq!(to.count(), 1);
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = self.0;

        LowerHex::fmt(&val, f)
    }
}

// TODO: This is slow as fuck
impl Iterator for Bitboard {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let lsb = self.0.trailing_zeros();
        self.0 &= self.0 - 1;
        Some(Bitboard(1 << lsb))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitboardError {
    InvalidSingleSquare(String),
    NoPieceAtSquare(Bitboard),
    TryFromIntError(TryFromIntError),
}

use std::num::TryFromIntError;

use crate::piece::Color;

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
