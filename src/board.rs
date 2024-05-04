#![allow(dead_code, unused)]
use crate::{
    bitboard::Bitboard,
    piece::{self, piece_to_letter, Color, Piece, PieceKind},
    printer,
    r#move::Move,
};
use std::fmt::Write;

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
            Some(Piece::new(color, PieceKind::Pawn))
        } else if !(square & self.knights).is_empty() {
            Some(Piece::new(color, PieceKind::Knight))
        } else if !(square & self.bishops).is_empty() {
            Some(Piece::new(color, PieceKind::Bishop))
        } else if !(square & self.rooks).is_empty() {
            Some(Piece::new(color, PieceKind::Rook))
        } else if !(square & self.queens).is_empty() {
            Some(Piece::new(color, PieceKind::Queen))
        } else if !(square & self.kings).is_empty() {
            Some(Piece::new(color, PieceKind::King))
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
                PieceKind::Pawn => {
                    self.pawns.clear_bit(mov.to);
                }
                PieceKind::Knight => {
                    self.knights.clear_bit(mov.to);
                }
                PieceKind::Bishop => {
                    self.bishops.clear_bit(mov.to);
                }
                PieceKind::Rook => {
                    self.rooks.clear_bit(mov.to);
                }
                PieceKind::Queen => {
                    self.queens.clear_bit(mov.to);
                }
                PieceKind::King => {
                    self.kings.clear_bit(mov.to);
                }
            }
        }

        let (mut color_mask, mut opposite_mask) = match piece.color {
            Color::White => (&mut self.white, &mut self.black),
            Color::Black => (&mut self.black, &mut self.white),
        };

        // TODO: handle castling
        match piece.kind {
            PieceKind::Pawn => {
                self.pawns.move_bit(mov.from, mov.to);
                // TODO: make promotions
            }
            PieceKind::Knight => {
                self.knights.move_bit(mov.from, mov.to);
            }
            PieceKind::Bishop => {
                self.bishops.move_bit(mov.from, mov.to);
            }
            PieceKind::Rook => {
                self.rooks.move_bit(mov.from, mov.to);
            }
            PieceKind::Queen => {
                self.queens.move_bit(mov.from, mov.to);
            }
            PieceKind::King => {
                self.kings.move_bit(mov.from, mov.to);
            }
        }
        color_mask.move_bit(mov.from, mov.to);

        #[cfg(debug_assertions)]
        {
            self.assert_sync();
        }
    }

    pub fn make_move(&mut self, mov: Move) {
        // let Some(piece) = self.get_piece(mov.from) else {
        //     println!("{}", self);
        //     panic!("No piece found at square: {}", mov.from);
        // };
        //

        self.move_piece(mov);

        // if it was a capture, remove the captured piece
        if let Some(capture) = mov.capture {
            match capture.kind {
                PieceKind::Pawn => {
                    self.pawns.clear_bit(mov.to);
                }
                PieceKind::Knight => {
                    self.knights.clear_bit(mov.to);
                }
                PieceKind::Bishop => {
                    self.bishops.clear_bit(mov.to);
                }
                PieceKind::Rook => {
                    self.rooks.clear_bit(mov.to);
                }
                PieceKind::Queen => {
                    self.queens.clear_bit(mov.to);
                }
                PieceKind::King => {
                    self.kings.clear_bit(mov.to);
                }
            }

            let mut color_mask = match capture.color {
                Color::White => &mut self.white,
                Color::Black => &mut self.black,
            };
            color_mask.clear_bit(mov.to);
        }

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
            PieceKind::Pawn => {
                self.pawns |= square;
            }
            PieceKind::Knight => {
                self.knights |= square;
            }
            PieceKind::Bishop => {
                self.bishops |= square;
            }
            PieceKind::Rook => {
                self.rooks |= square;
            }
            PieceKind::Queen => {
                self.queens |= square;
            }
            PieceKind::King => {
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
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Bitboard::FROM_SQUARE([file, rank]);
                let piece = self.get_piece(square);
                board += &format!("{} ", piece_to_letter(piece));
            }
            board += "\n";
        }
        write!(f, "{board}")
    }
}
