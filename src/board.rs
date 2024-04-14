use crate::piece::{Color, Piece, PieceKind};
use crate::printer;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const MAX: Bitboard = Bitboard(0xFF_FF_FF_FF_FF_FF_FF_FF);
    pub const FROM_SQUARE: fn([u8; 2]) -> Bitboard =
        |[file, rank]| Bitboard(1 << (rank * 8 + file));
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display: Vec<String> = printer::display_bitboard(self);
        let formatted = display.iter().fold(String::new(), |mut acc, s| {
            writeln!(acc, "{}", s).unwrap();
            acc
        });
        write!(f, "\n{}", formatted)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Bitboard(self.0 | rhs.0)
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

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::Shl<u64> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self {
        Bitboard(self.0 << rhs)
    }
}

impl std::ops::Shr<u64> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self {
        Bitboard(self.0 >> rhs)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Bitboard(!self.0)
    }
}

impl std::ops::BitXor<usize> for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self {
        Bitboard(self.0 ^ rhs as u64)
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Bitboard(self.0 ^ rhs.0)
    }
}

// Little-endian rank-file mapping
const FILE_A: Bitboard = Bitboard(0x80_80_80_80_80_80_80_80);
const NOT_FILE_A: Bitboard = Bitboard(0x7f_7f_7f_7f_7f_7f_7f_7f);
const FILE_H: Bitboard = Bitboard(0x01_01_01_01_01_01_01_01);
const NOT_FILE_H: Bitboard = Bitboard(0xfe_fe_fe_fe_fe_fe_fe_fe);

// A-H
const FILES: [Bitboard; 8] = [
    Bitboard(0x01_01_01_01_01_01_01_01),
    Bitboard(0x02_02_02_02_02_02_02_02),
    Bitboard(0x04_04_04_04_04_04_04_04),
    Bitboard(0x08_08_08_08_08_08_08_08),
    Bitboard(0x10_10_10_10_10_10_10_10),
    Bitboard(0x20_20_20_20_20_20_20_20),
    Bitboard(0x40_40_40_40_40_40_40_40),
    Bitboard(0x80_80_80_80_80_80_80_80),
];

// 1-8
const RANKS: [Bitboard; 8] = [
    Bitboard(0x00_00_00_00_00_00_00_FF),
    Bitboard(0x00_00_00_00_00_00_FF_00),
    Bitboard(0x00_00_00_00_00_FF_00_00),
    Bitboard(0x00_00_00_00_FF_00_00_00),
    Bitboard(0x00_00_00_00_FF_00_00_00),
    Bitboard(0x00_00_00_FF_00_00_00_00),
    Bitboard(0x00_00_FF_00_00_00_00_00),
    Bitboard(0x00_FF_00_00_00_00_00_00),
];

const RANK_1: Bitboard = RANKS[0];
const NOT_RANK_1: Bitboard = Bitboard(0xFF_FF_FF_FF_FF_FF_FF_00);
const RANK_8: Bitboard = RANKS[7];
const NOT_RANK_8: Bitboard = Bitboard(0x00_FF_FF_FF_FF_FF_FF_FF);

const PAWN_INITIAL: Bitboard = Bitboard(0x00_FF_00_00_00_00_FF_00);

fn north(bb: Bitboard) -> Bitboard {
    (bb & NOT_RANK_8) << 8
}

fn south(bb: Bitboard) -> Bitboard {
    (bb & NOT_RANK_1) >> 8
}

fn east(bb: Bitboard) -> Bitboard {
    (bb & NOT_FILE_H) << 1
}

fn west(bb: Bitboard) -> Bitboard {
    (bb & NOT_FILE_A) >> 1
}

fn north_east(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_8 | NOT_FILE_H)) << 9
}

fn north_west(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_8 | NOT_FILE_A)) << 7
}

fn south_east(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_1 | NOT_FILE_H)) >> 7
}

fn south_west(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_1 | NOT_FILE_A)) >> 9
}

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
    const DEFAULT: Board = Board {
        pawns: Bitboard(0),
        knights: Bitboard(0),
        bishops: Bitboard(0),
        rooks: Bitboard(0),
        queens: Bitboard(0),
        kings: Bitboard(0),
        white: Bitboard(0),
        black: Bitboard(0),
        en_passant: None,
        castling: 0,
        halfmove_clock: 0,
        fullmove_number: 1,
        side_to_move: Color::White,
    };
    pub const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
    pub fn new(fen: &str) -> Self {
        let mut board = Self::DEFAULT;
        let mut rank = 7;
        let mut file = 0;
        let pieces = match fen.split(' ').nth(0) {
            Some(pieces) => pieces,
            None => {
                panic!("Invalid FEN string: {}", fen);
            }
        };
        for c in pieces.chars() {
            match c {
                'P' => {
                    board.pawns |= Bitboard::FROM_SQUARE([file, rank]);
                    board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'N' => {
                    board.knights |= Bitboard::FROM_SQUARE([file, rank]);
                    board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'B' => {
                    board.bishops |= Bitboard::FROM_SQUARE([file, rank]);
                    board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'R' => {
                    board.rooks |= Bitboard::FROM_SQUARE([file, rank]);
                    board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'Q' => {
                    board.queens |= Bitboard::FROM_SQUARE([file, rank]);
                    board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'K' => {
                    board.kings |= Bitboard::FROM_SQUARE([file, rank]);
                    board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'p' => {
                    board.pawns |= Bitboard::FROM_SQUARE([file, rank]);
                    board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'n' => {
                    board.knights |= Bitboard::FROM_SQUARE([file, rank]);
                    board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'b' => {
                    board.bishops |= Bitboard::FROM_SQUARE([file, rank]);
                    board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'r' => {
                    board.rooks |= Bitboard::FROM_SQUARE([file, rank]);
                    board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'q' => {
                    board.queens |= Bitboard::FROM_SQUARE([file, rank]);
                    board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'k' => {
                    board.kings |= Bitboard::FROM_SQUARE([file, rank]);
                    board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                '1'..='8' => file += c as u8 - b'0',
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                _ => {
                    panic!("Invalid FEN character: {}", c);
                }
            }
        }
        board
    }

    pub fn get_piece(&self, square: Bitboard) -> Option<Piece> {
        if self.pawns & square != Bitboard(0) {
            Some(Piece::new(Color::White, PieceKind::Pawn))
        } else if self.knights & square != Bitboard(0) {
            Some(Piece::new(Color::White, PieceKind::Knight))
        } else if self.bishops & square != Bitboard(0) {
            Some(Piece::new(Color::White, PieceKind::Bishop))
        } else if self.rooks & square != Bitboard(0) {
            Some(Piece::new(Color::White, PieceKind::Rook))
        } else if self.queens & square != Bitboard(0) {
            Some(Piece::new(Color::White, PieceKind::Queen))
        } else if self.kings & square != Bitboard(0) {
            Some(Piece::new(Color::White, PieceKind::King))
        } else if self.black & square != Bitboard(0) {
            Some(Piece::new(Color::Black, PieceKind::Pawn))
        } else {
            None
        }
    }

    // pub fn is_occupied(&self, square: Bitboard, color: Color) -> bool {
    //     match color {
    //         Color::White => self.white & square != Bitboard(0),
    //         Color::Black => self.black & square != Bitboard(0),
    //     }
    // }
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
                board += &format!("{} ", c);
            }
            board += "\n";
        }
        write!(f, "{}", board)
    }
}

pub struct Move {
    pub from: Bitboard,
    pub to: Bitboard,
    pub promotion: Option<PieceKind>,
}

impl Move {
    pub fn new(from: Bitboard, to: Bitboard) -> Self {
        Self {
            from,
            to,
            promotion: None,
        }
    }
    // only pawns can promote
    pub fn with_promotion(from: Bitboard, to: Bitboard, promotion: PieceKind) -> Self {
        #[cfg(debug_assertions)]
        {
            assert_ne!(promotion, PieceKind::Pawn, "Cannot promote to pawn");
        }

        Self {
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let from_display: Vec<String> = printer::display_bitboard(&self.from);
        let to_display: Vec<String> = printer::display_bitboard(&self.to);
        writeln!(f, "from:              to:")?;
        // let format = |s: &str| s.chars().map(|c| format!("{} ", c)).collect::<String>();
        let format = |s: &str| {
            s.chars().fold(String::new(), |mut output, c| {
                let _ = write!(output, "{} ", c);
                output
            })
        };
        // let formatted: String = from_display
        //     .iter()
        //     .zip(to_display.iter())
        //     .map(|(from, to)| format!("{} | {}", format(from), format(to)))
        //     .collect::<Vec<String>>()
        //     .join("\n");
        let formatted: String = from_display.iter().zip(to_display.iter()).fold(
            String::new(),
            |mut acc, (from, to)| {
                if !acc.is_empty() {
                    writeln!(acc).unwrap(); // Safely append a newline if not the first entry
                }
                write!(acc, "{} | {}", format(from), format(to)).unwrap(); // Write formatted string directly to accumulator
                acc
            },
        );
        write!(f, "{}", formatted)?;
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
            let to = north(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
            }
        }
        Direction::South => {
            let to = south(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
            }
        }
        Direction::East => {
            let to = east(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
            }
        }
        Direction::West => {
            let to = west(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
            }
        }
        Direction::NorthEast => {
            let to = north_east(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
            }
        }
        Direction::NorthWest => {
            let to = north_west(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
            }
        }
        Direction::SouthEast => {
            let to = south_east(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
            }
        }
        Direction::SouthWest => {
            let to = south_west(origin_square);
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to));
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
            let to: Bitboard = north(origin_square);
            moves.push(Move::new(origin_square, to));
            if origin_square & PAWN_INITIAL != Bitboard(0) {
                moves.push(Move::new(origin_square, north(to)));
            }
            // TODO: capture moves
            moves
        }
        PieceKind::Knight => {
            let mut moves: Vec<Move> = vec![];
            KNIGHT_MOVES.iter().for_each(|&offset| {
                let positive = origin_square << offset.into();
                if positive & current_turn_mask != Bitboard(0) {
                    moves.push(Move::new(origin_square, positive));
                }
                let negative = origin_square >> offset.into();
                if negative & current_turn_mask != Bitboard(0) {
                    moves.push(Move::new(origin_square, negative));
                }
            });
            moves
        }
        PieceKind::Bishop => {
            let mut moves: Vec<Move> = vec![];
            [
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ]
            .iter()
            .for_each(|direction| {
                gen_sliding_moves(&mut moves, board, &piece, origin_square, direction);
            });
            moves
        }
        PieceKind::Rook => {
            let mut moves: Vec<Move> = vec![];
            [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ]
            .iter()
            .for_each(|direction| {
                gen_sliding_moves(&mut moves, board, &piece, origin_square, direction);
            });
            moves
        }
        PieceKind::Queen => {
            let mut moves: Vec<Move> = vec![];
            [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ]
            .iter()
            .for_each(|direction| {
                gen_sliding_moves(&mut moves, board, &piece, origin_square, direction);
            });
            moves
        }
        PieceKind::King => {
            let mut moves: Vec<Move> = vec![];
            [
                north(origin_square),
                south(origin_square),
                east(origin_square),
                west(origin_square),
                north_east(origin_square),
                north_west(origin_square),
                south_east(origin_square),
                south_west(origin_square),
            ]
            .iter()
            .filter(|&to| *to != Bitboard(0) && *to & current_turn_mask == Bitboard(0))
            .for_each(|&to| moves.push(Move::new(origin_square, to)));
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
                    None => panic!("No piece found at square: {}", i),
                };
            }
            let mut piece_moves = gen_moves_from_piece(board, square);
            moves.append(&mut piece_moves);
        }
    }

    moves
}

// pub fn make_move(board: &mut Board, mov: Move) {}
