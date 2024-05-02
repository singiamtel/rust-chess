#![allow(dead_code, unused)]
use crate::{
    bitboard::{Bitboard, RANK_1, RANK_8},
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
            Some(Piece::new(self.get_color(square).unwrap(), PieceKind::Pawn))
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

    pub fn make_move(board: &mut Self, mov: Move) {
        let Some(piece) = board.get_piece(mov.from) else {
            panic!("No piece found at square: {}", mov.from);
        };
        let mut color_mask = match piece.color {
            Color::White => board.white,
            Color::Black => board.black,
        };

        match piece.kind {
            PieceKind::Pawn => {
                board.pawns.move_bit(mov.from, mov.to);
                // TODO: make promotions
            }
            PieceKind::Knight => {
                board.knights.move_bit(mov.from, mov.to);
            }
            PieceKind::Bishop => {
                board.bishops.move_bit(mov.from, mov.to);
            }
            PieceKind::Rook => {
                board.rooks.move_bit(mov.from, mov.to);
            }
            PieceKind::Queen => {
                board.queens.move_bit(mov.from, mov.to);
            }
            PieceKind::King => {
                board.kings.move_bit(mov.from, mov.to);
                // TODO: handle castling
            }
        }
        color_mask.move_bit(mov.from, mov.to);
        // if it was a capture, remove the captured piece
        // TODO: handle en passant
        if mov.capture.is_some() {
            let captured_piece = board.get_piece(mov.to).unwrap();
            match captured_piece.kind {
                PieceKind::Pawn => {
                    board.pawns.clear_bit(mov.to);
                }
                PieceKind::Knight => {
                    board.knights.clear_bit(mov.to);
                }
                PieceKind::Bishop => {
                    board.bishops.clear_bit(mov.to);
                }
                PieceKind::Rook => {
                    board.rooks.clear_bit(mov.to);
                }
                PieceKind::Queen => {
                    board.queens.clear_bit(mov.to);
                }
                PieceKind::King => {
                    board.kings.clear_bit(mov.to);
                }
            }
            color_mask.clear_bit(mov.to);
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Bitboard::FROM_SQUARE([file, rank]);
                let c = match self.get_piece(square) {
                    Some(piece) => match piece.kind {
                        PieceKind::Pawn => 'P',
                        PieceKind::Knight => 'N',
                        PieceKind::Bishop => 'B',
                        PieceKind::Rook => 'R',
                        PieceKind::Queen => 'Q',
                        PieceKind::King => 'K',
                    },
                    None => '.',
                };
                board += &format!("{c} ");
            }
            board += "\n";
        }
        write!(f, "{board}")
    }
}
