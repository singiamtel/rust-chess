#![allow(dead_code, unused)]
use crate::{
    bitboard::{Bitboard, RANK_1, RANK_8},
    piece::{Color, Piece, PieceKind},
    printer,
};
use std::fmt::Write;

// Little-endian rank-file mapping

const KNIGHT_MOVES: [u8; 4] = [17, 15, 10, 6];

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

    pub en_passant: Option<Bitboard>,
    pub castling: u8,

    pub halfmove_clock: u8,
    pub fullmove_number: u16,

    pub side_to_move: Color,
}

impl Board {
    pub fn get_color(self, square: Bitboard) -> Option<Color> {
        if !(square & self.white).is_empty() {
            Some(Color::White)
        } else if !(square & self.black).is_empty() {
            Some(Color::Black)
        } else {
            None
        }
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

pub struct Move {
    pub piece: Piece,
    pub from: Bitboard,
    pub to: Bitboard,
    pub promotion: Option<PieceKind>,
    pub en_passant: Option<Bitboard>,
    pub castling: u8, // Keep track of changes to castling rights
    pub capture: Option<PieceKind>,
}

impl Move {
    pub fn new(piece: &Piece, from: Bitboard, to: Bitboard, promotion: Option<PieceKind>) -> Self {
        #[cfg(debug_assertions)]
        {
            assert!(
                !(from & to != Bitboard(0)),
                "From and to squares are the same"
            );
            assert!(
                !(piece.kind != PieceKind::Pawn && promotion.is_some()),
                "Non-pawn piece cannot promote"
            );
            assert!(
                !(to & (RANK_1 | RANK_8) == Bitboard(0) && promotion.is_some()),
                "Pawn must promote on rank 1 or 8"
            );
        }
        Self {
            from,
            to,
            promotion,
            en_passant: None,
            castling: 0,
            capture: None,
            piece: Piece::new(Color::White, PieceKind::Pawn),
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let from_display: Vec<String> = printer::display_bitboard(self.from);
        let to_display: Vec<String> = printer::display_bitboard(self.to);
        writeln!(f, "from:              to:")?;
        let format: fn(&str) -> String = |s: &str| -> String {
            s.chars().fold(String::new(), |mut output, c| -> String {
                write!(output, "{c} ");
                output
            })
        };
        let formatted: String = from_display.iter().zip(to_display.iter()).fold(
            String::new(),
            |mut acc, (from, to)| -> String {
                if !acc.is_empty() {
                    writeln!(acc);
                }
                write!(acc, "{} | {}", format(from), format(to));
                acc
            },
        );
        write!(f, "{formatted}");
        Ok(())
    }
}

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

pub fn gen_sliding_moves(
    moves: &mut Vec<Move>,
    board: &Board,
    piece: &Piece,
    origin_square: Bitboard,
    direction: &Direction,
) {
    let current_turn_mask = if piece.color == Color::White {
        board.white
    } else {
        board.black
    };
    match direction {
        Direction::North => {
            let to = origin_square.north();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
        Direction::South => {
            let to = origin_square.south();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
        Direction::East => {
            let to = origin_square.east();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
        Direction::West => {
            let to = origin_square.west();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
        Direction::NorthEast => {
            let to = origin_square.north_east();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
        Direction::NorthWest => {
            let to = origin_square.north_west();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
        Direction::SouthEast => {
            let to = origin_square.south_east();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
        Direction::SouthWest => {
            let to = origin_square.south_west();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(piece, origin_square, to, None));
            }
        }
    }
}

// pseudo-legal moves
// Does not check for check or pinned pieces
pub fn gen_moves_from_piece(board: &Board, origin_square: Bitboard) -> Vec<Move> {
    let piece = match board.get_piece(origin_square) {
        Some(piece) => piece,
        None => return vec![],
    };
    let current_turn_mask = if piece.color == Color::White {
        board.white
    } else {
        board.black
    };
    let moves: Vec<Move> = match piece.kind {
        PieceKind::Pawn => {
            let mut moves: Vec<Move> = vec![];
            let to: Bitboard = origin_square.north();
            moves.push(Move::new(&piece, origin_square, to, None));
            if origin_square.pawn_initial(current_turn_mask) {
                moves.push(Move::new(&piece, origin_square, to.north(), None));
            }
            // TODO: capture moves, en passant, and promotion
            moves
        }
        PieceKind::Knight => {
            let mut moves: Vec<Move> = vec![];
            for &offset in &KNIGHT_MOVES {
                let positive = origin_square << offset.into();
                if positive & current_turn_mask != Bitboard(0) {
                    moves.push(Move::new(&piece, origin_square, positive, None));
                }
                let negative = origin_square >> offset.into();
                if negative & current_turn_mask != Bitboard(0) {
                    moves.push(Move::new(&piece, origin_square, negative, None));
                }
            }
            moves
        }
        PieceKind::Bishop => {
            let mut moves: Vec<Move> = vec![];
            for direction in [
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ] {
                gen_sliding_moves(&mut moves, board, &piece, origin_square, &direction);
            }
            moves
        }
        PieceKind::Rook => {
            let mut moves: Vec<Move> = vec![];
            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                gen_sliding_moves(&mut moves, board, &piece, origin_square, &direction);
            }
            moves
        }
        PieceKind::Queen => {
            let mut moves: Vec<Move> = vec![];
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ] {
                gen_sliding_moves(&mut moves, board, &piece, origin_square, direction);
            }
            moves
        }
        PieceKind::King => {
            let mut moves: Vec<Move> = vec![];
            [
                origin_square.north(),
                origin_square.south(),
                origin_square.east(),
                origin_square.west(),
                origin_square.north_east(),
                origin_square.north_west(),
                origin_square.south_east(),
                origin_square.south_west(),
            ]
            .iter()
            .filter(|&to| *to != Bitboard(0) && *to & current_turn_mask == Bitboard(0))
            .for_each(|&to| moves.push(Move::new(&piece, origin_square, to, None)));
            moves
        }
    };
    moves
}

pub fn gen_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let occupied =
        board.pawns | board.knights | board.bishops | board.rooks | board.queens | board.kings;

    let current_turn_mask = if board.side_to_move == Color::White {
        board.white
    } else {
        board.black
    };
    for i in 0..64 {
        let square = Bitboard(1 << i);
        if occupied & square & current_turn_mask != Bitboard(0) {
            #[cfg(debug_assertions)]
            {
                match board.get_piece(square) {
                    Some(piece) => piece,
                    None => panic!("No piece found at square: {i}"),
                };
            }
            let mut piece_moves = gen_moves_from_piece(board, square);
            moves.append(&mut piece_moves);
        }
    }

    moves
}

pub fn make_move(board: &mut Board, mov: Move) {
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
        }
    }
    color_mask.move_bit(mov.from, mov.to);
}
