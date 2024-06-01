use std::fmt::{Display, Formatter, Result};

use crate::bitboard::display::BitboardDisplay;
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
    pub const WHITE_KINGSIDE: u8 = 0b1000;
    pub const WHITE_QUEENSIDE: u8 = 0b0100;
    pub const BLACK_KINGSIDE: u8 = 0b0010;
    pub const BLACK_QUEENSIDE: u8 = 0b0001;

    // Set or clear specific castling rights
    pub fn set_castling_right(&mut self, right: u8, allowed: bool) {
        if allowed {
            self.0 |= right;
        } else {
            self.0 &= !right;
        }
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

    pub king_position: OnePerColor<Option<usize>>,
    pub en_passant: Option<Bitboard>,

    pub castling: CastlingRights,
}

impl Board {
    pub const DEFAULT: Self = Self {
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
        castling: CastlingRights(0),
    };
    pub fn get_color(self, square: Bitboard) -> Option<Color> {
        if !(square & self.white).is_empty() {
            Some(Color::White)
        } else if !(square & self.black).is_empty() {
            Some(Color::Black)
        } else {
            None
        }
    }
    pub fn occupied(self) -> Bitboard {
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

    pub fn clear_piece(&mut self, piece: Kind, color: Color, square: Bitboard) {
        let color_mask = match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };
        color_mask.clear_bit(square);
        match piece {
            Kind::Pawn => self.pawns.clear_bit(square),
            Kind::Knight => self.knights.clear_bit(square),
            Kind::Bishop => self.bishops.clear_bit(square),
            Kind::Rook => self.rooks.clear_bit(square),
            Kind::Queen => self.queens.clear_bit(square),
            Kind::King => {
                self.kings.clear_bit(square);
                match color {
                    Color::White => self.king_position.white = None,
                    Color::Black => self.king_position.black = None,
                }
            }
        }
    }
    pub fn move_piece(&mut self, mov: Move) {
        #[cfg(debug_assertions)]
        {
            assert!(
                self.get_piece(mov.from).is_some(),
                "No piece found at square: {}\n{self}",
                mov.from
            );
        }
        let piece = mov.what;
        if let Some(en_passant) = mov.en_passant {
            self.en_passant = Some(en_passant);
        } else {
            self.en_passant = None;
        }

        // We handle capture first, so we don't face issues when trying to eat a piece of the same
        // type

        // if let Some(en_passant) = mov.en_passant {
        //     // remove the pawn that would be captured by en passant
        //     self.clear_piece(Kind::Pawn, piece.color, en_passant);
        // }
        if let Some(capture) = mov.capture {
            self.clear_piece(capture.kind, capture.color, capture.position);
        }

        let color_mask = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };

        // TODO: handle castling
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
        assert_eq!(
            self.white & self.occupied(),
            self.white,
            "White and occupied are out of sync"
        );
        assert_eq!(
            self.black & self.occupied(),
            self.black,
            "Black and occupied are out of sync"
        );
    }

    pub fn unmove_piece(&mut self, mov: Move) {
        self.move_piece(Move::new(mov.to, mov.from, mov.what));
    }

    pub fn spawn_piece(&mut self, piece: Piece) {
        let color_mask = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };

        let position = piece.position;

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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Bitboard::from_square(file, rank);
                let piece = self.get_piece(square);
                board += &format!("{} ", to_letter(piece));
            }
            board += "\n";
        }
        write!(f, "{board}")
    }
}
