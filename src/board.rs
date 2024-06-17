#![allow(dead_code, unused_imports)]

use std::fmt::{Display, Formatter, LowerHex, Result};
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXorAssign, Not};

use crate::bitboard::display::BitboardDisplay;
use crate::bitboard::{generate_knight_lookup, generate_pawn_lookup, Direction};
use crate::move_generation::Movegen;

use crate::{
    bitboard::{Bitboard, DirectionalShift},
    piece::{to_letter, Color, Kind, Piece},
    r#move::Move,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnePerColor<T> {
    pub white: T,
    pub black: T,
}

impl<T> OnePerColor<T> {
    pub const fn new(white: T, black: T) -> Self {
        Self { white, black }
    }
    pub fn get(&self, color: Color) -> &T {
        match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    // Constants for each castling right bit
    pub const WHITE_KINGSIDE: Self = CastlingRights(0b1000);
    pub const WHITE_QUEENSIDE: Self = CastlingRights(0b0100);
    pub const BLACK_KINGSIDE: Self = CastlingRights(0b0010);
    pub const BLACK_QUEENSIDE: Self = CastlingRights(0b0001);
    pub const WHITE_BOTH: Self = CastlingRights(0b1100);
    pub const BLACK_BOTH: Self = CastlingRights(0b0011);
    pub const ALL: Self = CastlingRights(0b1111);
    pub const NONE: Self = CastlingRights(0b0000);

    // Set or clear specific castling rights
    pub fn set_castling_right(&mut self, right: Self, allowed: bool) {
        if allowed {
            *self |= right;
        } else {
            *self &= !right;
        }
    }

    pub fn toggle_right(&mut self, right: Self) {
        *self ^= right;
    }

    pub fn get_castling_right(self, right: Self) -> bool {
        self & right != Self::NONE
    }

    #[inline(always)]
    pub const fn white_queenside_squares() -> Bitboard {
        Bitboard(0xe)
    }

    #[inline(always)]
    pub const fn white_kingside_squares() -> Bitboard {
        Bitboard(0x60)
    }
    #[inline(always)]
    pub const fn black_queenside_squares() -> Bitboard {
        Bitboard(0xe00000000000000)
    }

    #[inline(always)]
    pub const fn black_kingside_squares() -> Bitboard {
        Bitboard(0x6000000000000000)
    }
}

impl BitAnd<CastlingRights> for CastlingRights {
    type Output = CastlingRights;
    fn bitand(self, rhs: Self) -> Self {
        CastlingRights(self.0 & rhs.0)
    }
}

impl BitOrAssign for CastlingRights {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for CastlingRights {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitAndAssign for CastlingRights {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for CastlingRights {
    type Output = CastlingRights;
    fn not(self) -> Self {
        CastlingRights(!self.0)
    }
}

impl LowerHex for CastlingRights {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = self.0;

        LowerHex::fmt(&val, f)
    }
}

// Little-endian rank-file mapping

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queens: Bitboard,
    pub kings: Bitboard,
    pub white: Bitboard,
    pub black: Bitboard,

    pub turn: Color,

    pub king_position: OnePerColor<Option<usize>>,
    pub en_passant: Option<Bitboard>,

    pub attacked_squares: Bitboard,
    pub pawn_attacks_lookup: OnePerColor<[Bitboard; 64]>,
    pub knight_attacks_lookup: [Bitboard; 64],

    pub castling: CastlingRights,
}

impl Board {
    pub fn new() -> Self {
        let pawn_attacks_lookup = generate_pawn_lookup();
        let knight_attacks_lookup = generate_knight_lookup();
        let pawn_attacks_lookup = OnePerColor::new(pawn_attacks_lookup[0], pawn_attacks_lookup[1]);
        Self {
            pawns: Bitboard(0),
            knights: Bitboard(0),
            bishops: Bitboard(0),
            rooks: Bitboard(0),
            queens: Bitboard(0),
            kings: Bitboard(0),
            white: Bitboard(0),
            black: Bitboard(0),
            king_position: OnePerColor::new(None, None),
            en_passant: None,
            attacked_squares: Bitboard(0),
            pawn_attacks_lookup,
            knight_attacks_lookup,
            castling: CastlingRights(0),

            turn: Color::White,
        }
    }

    pub fn king_position(&self, color: Color) -> usize {
        match color {
            Color::White => self.king_position.white.expect("King position not set"),
            Color::Black => self.king_position.black.expect("King position not set"),
        }
    }

    pub fn get_color(self, square: Bitboard) -> Option<Color> {
        if !(square & self.white).is_empty() {
            Some(Color::White)
        } else if !(square & self.black).is_empty() {
            Some(Color::Black)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn anything(&self) -> Bitboard {
        self.black | self.white
    }

    pub fn get_piece(&self, square: Bitboard) -> Option<Piece> {
        let Some(color) = self.get_color(square) else {
            return None;
        };
        if !(square & self.pawns).is_empty() {
            Some(Piece::new(color, Kind::Pawn, square))
        } else if !(square & self.knights).is_empty() {
            Some(Piece::new(color, Kind::Knight, square))
        } else if !(square & self.bishops).is_empty() {
            Some(Piece::new(color, Kind::Bishop, square))
        } else if !(square & self.rooks).is_empty() {
            Some(Piece::new(color, Kind::Rook, square))
        } else if !(square & self.queens).is_empty() {
            Some(Piece::new(color, Kind::Queen, square))
        } else if !(square & self.kings).is_empty() {
            Some(Piece::new(color, Kind::King, square))
        } else {
            None
        }
    }

    pub fn get_en_passant_victim(&self, en_passant_square: Bitboard, color: Color) -> Piece {
        // the en passant square is the capturable square, but the pawn is in
        // either the next or previous rank, depending on the turn
        match color {
            Color::White => {
                let pawn_square = en_passant_square.north();
                let piece = self.get_piece(pawn_square);
                if let Some(piece) = piece {
                    piece
                } else {
                    panic!(
                        "No en passant pawn found for {} at {}. En passant square: {}. Board: {}",
                        color,
                        pawn_square.to_algebraic().unwrap(),
                        en_passant_square.to_algebraic().unwrap(),
                        self
                    );
                }
            }
            Color::Black => {
                let pawn_square = en_passant_square.south();
                let piece = self.get_piece(pawn_square);
                if let Some(piece) = piece {
                    piece
                } else {
                    panic!(
                        "No en passant pawn found for {} at {}. En passant square: {}. Board: {}",
                        color,
                        pawn_square.to_algebraic().unwrap(),
                        en_passant_square.to_algebraic().unwrap(),
                        self
                    );
                }
            }
        }
    }

    pub fn clear_piece(&mut self, piece: Piece) {
        let color_mask = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };
        #[cfg(debug_assertions)]
        {
            assert!(
                color_mask.intersects(piece.position),
                "Tried to clear piece that is not on the board. {} {} {}",
                color_mask,
                piece.position,
                piece
            );
        }
        color_mask.clear_bit(piece.position);
        match piece.kind {
            Kind::Pawn => self.pawns.clear_bit(piece.position),
            Kind::Knight => self.knights.clear_bit(piece.position),
            Kind::Bishop => self.bishops.clear_bit(piece.position),
            Kind::Rook => self.rooks.clear_bit(piece.position),
            Kind::Queen => self.queens.clear_bit(piece.position),
            Kind::King => {
                self.kings.clear_bit(piece.position);
                match piece.color {
                    Color::White => self.king_position.white = None,
                    Color::Black => self.king_position.black = None,
                }
            }
        }
    }

    fn get_pieces(&self, kind: Kind, color: Color) -> Bitboard {
        let pieces = match kind {
            Kind::Pawn => self.pawns,
            Kind::Knight => self.knights,
            Kind::Bishop => self.bishops,
            Kind::Rook => self.rooks,
            Kind::Queen => self.queens,
            Kind::King => self.kings,
        };

        let color_mask = match color {
            Color::White => self.white,
            Color::Black => self.black,
        };

        pieces & color_mask
    }

    fn generate_pawn_attacks(&self, color: Color) -> Bitboard {
        let mut attacks = Bitboard(0);
        let pawns = self.get_pieces(Kind::Pawn, !color);
        // find all pawns
        for pawn in pawns {
            let pawn_idx = pawn.idx();
            let pawn_attack = self.pawn_attacks_lookup.get(color)[pawn_idx];
            attacks |= pawn_attack;
        }
        attacks
    }

    fn generate_knight_attacks(&self, color: Color) -> Bitboard {
        let mut attacks = Bitboard(0);
        let knights = self.get_pieces(Kind::Knight, !color);
        for knight in knights {
            let knight_idx = knight.idx();
            let knight_attack = self.knight_attacks_lookup[knight_idx];
            attacks |= knight_attack;
        }
        attacks
    }

    fn generate_bishop_attacks(&self, _color: Color) -> Bitboard {
        // let moves: Vec<Move> = Vec::new();
        // let bishop = self.get_piece(Kind::Bishop, !color);
        // self.gen_sliding_moves(&mut moves, bishop, bishop, Direction::NorthEast);
        // self.gen_sliding_moves(&mut moves, bishop, bishop, Direction::NorthWest);
        // self.gen_sliding_moves(&mut moves, bishop, bishop, Direction::SouthEast);
        // self.gen_sliding_moves(&mut moves, bishop, bishop, Direction::SouthWest);
        // moves
        Bitboard(0)
    }

    fn calculate_attacked_squares(&self) -> Bitboard {
        let mut attacks = Bitboard(0);
        // pawns
        attacks |= self.generate_pawn_attacks(self.turn);

        // knights
        attacks |= self.generate_knight_attacks(self.turn);

        // bishops
        // rooks
        // queens
        // king
        attacks
    }

    pub fn flip_turn(&mut self) {
        self.turn = !self.turn;
    }

    pub fn move_piece(&mut self, mov: Move) {
        #[cfg(debug_assertions)]
        {
            assert!(
                self.get_piece(mov.from).is_some(),
                "No piece found at origin square for move {mov}\n{self}",
            );
        }
        let piece = mov.what;
        if let Some(en_passant) = mov.en_passant {
            self.en_passant = Some(en_passant);
        } else {
            self.en_passant = None;
        }

        if let Some(castle_move) = mov.castle_move {
            // TODO: move it instead
            self.clear_piece(Piece::new(piece.color, Kind::Rook, castle_move.0));
            self.spawn_piece(Piece::new(piece.color, Kind::Rook, castle_move.1));
            self.castling.toggle_right(mov.castling_rights_change);
        }

        // We handle capture first, so we don't face issues when trying to eat a piece of the same
        // type
        if let Some(capture) = mov.capture {
            self.clear_piece(capture);
        }

        let color_mask = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };

        match piece.kind {
            Kind::Pawn => {
                self.pawns.move_bit(mov.from, mov.to);
                // TODO: make promotions
            }
            Kind::Knight => {
                self.knights.move_bit(mov.from, mov.to);
            }
            Kind::Bishop => {
                self.bishops.move_bit(mov.from, mov.to);
            }
            Kind::Rook => {
                self.rooks.move_bit(mov.from, mov.to);
            }
            Kind::Queen => {
                self.queens.move_bit(mov.from, mov.to);
            }
            Kind::King => {
                self.kings.move_bit(mov.from, mov.to);
                match piece.color {
                    Color::White => self.king_position.white = Some(mov.to.idx()),
                    Color::Black => self.king_position.black = Some(mov.to.idx()),
                }
                // self.castling &= !(1 << mov.to.idx());
            }
        }
        color_mask.move_bit(mov.from, mov.to);

        // self.attacked_squares = self.calculate_attacked_squares();

        #[cfg(debug_assertions)]
        {
            self.assert_sync();
        }
    }

    pub fn assert_sync(&self) {
        // verify that color masks are correct
        assert_eq!(
            self.white & self.black,
            Bitboard(0),
            "White and black overlap {} {}",
            self.white,
            self.black
        );
        // TODO: check that inter-piece masks dont collide, and always intersect with color_masks
    }

    pub fn unmove_piece(&mut self, mov: Move) {
        self.move_piece(Move::new(mov.to, mov.from, mov.what));
        // restore old piece
        if let Some(captured_piece) = mov.capture {
            self.spawn_piece(captured_piece);
        }

        if let Some(castle_move) = mov.castle_move {
            // TODO: move it instead
            self.clear_piece(Piece::new(mov.what.color, Kind::Rook, castle_move.1));
            self.spawn_piece(Piece::new(mov.what.color, Kind::Rook, castle_move.0));
            self.castling.toggle_right(mov.castling_rights_change);
        }
    }

    pub fn spawn_piece(&mut self, piece: Piece) {
        let color_mask = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };

        let position = piece.position;

        #[cfg(debug_assertions)]
        {
            assert!(
                !color_mask.intersects(piece.position),
                "Tried to spawn a piece on a busy square\n {} {} {}",
                self,
                piece.position,
                piece
            );
        }

        color_mask.set_bit(position);
        match piece.kind {
            Kind::Pawn => {
                self.pawns.set_bit(position);
            }
            Kind::Knight => {
                self.knights.set_bit(position);
            }
            Kind::Bishop => {
                self.bishops.set_bit(position);
            }
            Kind::Rook => {
                self.rooks.set_bit(position);
            }
            Kind::Queen => {
                self.queens.set_bit(position);
            }
            Kind::King => {
                self.kings.set_bit(position);
                match piece.color {
                    Color::White => self.king_position.white = Some(position.idx()),
                    Color::Black => self.king_position.black = Some(position.idx()),
                }
                #[cfg(debug_assertions)]
                {
                    assert!(
                        (self.kings & *color_mask).count() == 1,
                        "{} {} {}",
                        self.kings,
                        *color_mask,
                        (self.kings & *color_mask).count()
                    );
                    assert!(
                        self.king_position.get(piece.color).unwrap() == position.idx(),
                        "King position out of sync with king"
                    );
                }
            }
        }
    }

    pub fn get_color_mask(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

pub fn colorize(letter: char) -> String {
    // if check
    let answer: String = if letter.is_ascii_uppercase() {
        format!("\x1b[0;31m{}\x1b[0m", letter)
    } else {
        format!("\x1b[0;34m{}\x1b[0m", letter)
    };
    answer
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Bitboard::from_square(file, rank);
                let piece = self.get_piece(square);
                match piece {
                    Some(piece) => {
                        board += &format!("{} ", colorize(to_letter(Some(piece))));
                    }
                    None => {
                        if square & self.attacked_squares != Bitboard(0) {
                            board += "x ";
                        } else {
                            board += ". ";
                        }
                    }
                }
            }
            board += "\n";
        }
        write!(f, "{board}")
    }
}
