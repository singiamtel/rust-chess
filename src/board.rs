#![allow(dead_code, unused)]
use crate::{
    bitboard::Bitboard,
    piece::{Color, Piece, PieceKind},
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
        if !(square & self.pawns).is_empty() {
            Some(Piece::new(
                self.get_color(square)
                    .expect("Pieces and colors are out of sync"),
                PieceKind::Pawn,
            ))
        } else if !(square & self.knights).is_empty() {
            Some(Piece::new(
                self.get_color(square)
                    .expect("Pieces and colors are out of sync"),
                PieceKind::Knight,
            ))
        } else if !(square & self.bishops).is_empty() {
            Some(Piece::new(
                self.get_color(square)
                    .expect("Pieces and colors are out of sync"),
                PieceKind::Bishop,
            ))
        } else if !(square & self.rooks).is_empty() {
            Some(Piece::new(
                self.get_color(square)
                    .expect("Pieces and colors are out of sync"),
                PieceKind::Rook,
            ))
        } else if !(square & self.queens).is_empty() {
            Some(Piece::new(
                self.get_color(square)
                    .expect("Pieces and colors are out of sync"),
                PieceKind::Queen,
            ))
        } else if !(square & self.kings).is_empty() {
            Some(Piece::new(
                self.get_color(square)
                    .expect("Pieces and colors are out of sync"),
                PieceKind::King,
            ))
        } else {
            None
        }
    }

    pub fn move_piece(&mut self, mov: Move) {
        let Some(piece) = self.get_piece(mov.from) else {
            panic!("No piece found at square: {}", mov.from);
        };
        let mut color_mask = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
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
    }

    pub fn make_move(&mut self, mov: Move) {
        // let Some(piece) = self.get_piece(mov.from) else {
        //     println!("{}", self);
        //     panic!("No piece found at square: {}", mov.from);
        // };
        //

        self.move_piece(mov);

        // if it was a capture, remove the captured piece
        if mov.capture.is_some() {
            let captured_piece = self.get_piece(mov.to).unwrap();
            match captured_piece.kind {
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

            let mut color_mask = match self.get_color(mov.from) {
                Some(Color::White) => &mut self.white,
                Some(Color::Black) => &mut self.black,
                None => panic!("No piece found at square: {}", mov.from),
            };
            color_mask.clear_bit(mov.to);
        }
    }

    pub fn unmove_piece(&mut self, mov: Move) {
        self.move_piece(Move::new(mov.to, mov.from, None));
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Bitboard::FROM_SQUARE([file, rank]);
                let c = self
                    .get_piece(square)
                    .map_or('.', |piece| match piece.kind {
                        PieceKind::Pawn => 'P',
                        PieceKind::Knight => 'N',
                        PieceKind::Bishop => 'B',
                        PieceKind::Rook => 'R',
                        PieceKind::Queen => 'Q',
                        PieceKind::King => 'K',
                    });
                board += &format!("{c} ");
            }
            board += "\n";
        }
        write!(f, "{board}")
    }
}
