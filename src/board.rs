use std::fmt::{Display, Formatter, Result};

use crate::{
    bitboard::Bitboard,
    piece::{to_letter, Color, Kind, Piece},
    r#move::Move,
};

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
            Some(Piece::new(color, Kind::Pawn))
        } else if !(square & self.knights).is_empty() {
            Some(Piece::new(color, Kind::Knight))
        } else if !(square & self.bishops).is_empty() {
            Some(Piece::new(color, Kind::Bishop))
        } else if !(square & self.rooks).is_empty() {
            Some(Piece::new(color, Kind::Rook))
        } else if !(square & self.queens).is_empty() {
            Some(Piece::new(color, Kind::Queen))
        } else if !(square & self.kings).is_empty() {
            Some(Piece::new(color, Kind::King))
        } else {
            None
        }
    }

    pub fn move_piece(&mut self, mov: Move) {
        let Some(piece) = self.get_piece(mov.from) else {
            panic!("No piece found at square: {}\n{self}", mov.from);
        };

        // We handle capture first, so we don't face issues when trying to eat a piece of the same
        // type

        if let Some(capture) = mov.capture {
            let capture_color_mask = match capture.color {
                Color::White => &mut self.white,
                Color::Black => &mut self.black,
            };

            capture_color_mask.clear_bit(mov.to);
            match capture.kind {
                Kind::Pawn => {
                    self.pawns.clear_bit(mov.to);
                }
                Kind::Knight => {
                    self.knights.clear_bit(mov.to);
                }
                Kind::Bishop => {
                    self.bishops.clear_bit(mov.to);
                }
                Kind::Rook => {
                    self.rooks.clear_bit(mov.to);
                }
                Kind::Queen => {
                    self.queens.clear_bit(mov.to);
                }
                Kind::King => {
                    self.kings.clear_bit(mov.to);
                }
            }
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

    pub fn spawn_piece(&mut self, piece: Piece, square: Bitboard) {
        match piece.kind {
            Kind::Pawn => {
                self.pawns |= square;
            }
            Kind::Knight => {
                self.knights |= square;
            }
            Kind::Bishop => {
                self.bishops |= square;
            }
            Kind::Rook => {
                self.rooks |= square;
            }
            Kind::Queen => {
                self.queens |= square;
            }
            Kind::King => {
                self.kings |= square;
            }
        }
        match piece.color {
            Color::White => {
                self.white |= square;
            }
            Color::Black => {
                self.black |= square;
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
