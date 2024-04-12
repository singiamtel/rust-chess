use crate::piece::{Color, Piece, PieceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const MAX: Bitboard = Bitboard(0xFFFFFFFFFFFFFFFF);
    pub const from_square: fn([u8; 2]) -> Bitboard =
        |[file, rank]| Bitboard(1 << (rank * 8 + file));
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
const FILE_A: Bitboard = Bitboard(0x8080808080808080);
const NOT_FILE_A: Bitboard = Bitboard(0x7f7f7f7f7f7f7f7f);
const FILE_H: Bitboard = Bitboard(0x0101010101010101);
const NOT_FILE_H: Bitboard = Bitboard(0xfefefefefefefefe);

// A-H
const FILES: [Bitboard; 8] = [
    Bitboard(0x0101010101010101),
    Bitboard(0x0202020202020202),
    Bitboard(0x0404040404040404),
    Bitboard(0x0808080808080808),
    Bitboard(0x1010101010101010),
    Bitboard(0x2020202020202020),
    Bitboard(0x4040404040404040),
    Bitboard(0x8080808080808080),
];

// 1-8
const RANKS: [Bitboard; 8] = [
    Bitboard(0x00000000000000FF),
    Bitboard(0x0000000000000FF00),
    Bitboard(0x00000000000FF0000),
    Bitboard(0x0000000000FF00000),
    Bitboard(0x00000000FF0000000),
    Bitboard(0x0000FF0000000000),
    Bitboard(0x00FF000000000000),
    Bitboard(0xFF00000000000000),
];

const RANK_1: Bitboard = RANKS[0];
const NOT_RANK_1: Bitboard = Bitboard(0xFFFFFFFFFFFFF000);
const RANK_8: Bitboard = RANKS[7];
const NOT_RANK_8: Bitboard = Bitboard(0x00FFFFFFFFFFFFFF);

const PAWN_INITIAL: Bitboard = Bitboard(0x00FF00000000FF00);

fn north(bb: Bitboard) -> Bitboard {
    (bb & NOT_RANK_1) << 8
}

fn south(bb: Bitboard) -> Bitboard {
    (bb & NOT_RANK_8) >> 8
}

fn east(bb: Bitboard) -> Bitboard {
    (bb & NOT_FILE_A) << 1
}

fn west(bb: Bitboard) -> Bitboard {
    (bb & NOT_FILE_H) >> 1
}

fn north_east(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_1 | NOT_FILE_H)) << 7
}

fn north_west(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_1 | NOT_FILE_H)) << 7
}

fn south_east(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_8 | NOT_FILE_A)) >> 7
}

fn south_west(bb: Bitboard) -> Bitboard {
    (bb & (NOT_RANK_8 | NOT_FILE_A)) >> 7
}

const KNIGHT_MOVES: [(i8, i8); 8] = [
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
];

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
    const DEFAULT: Board = Board {
        pawns: Bitboard(0),
        knights: Bitboard(0),
        bishops: Bitboard(0),
        rooks: Bitboard(0),
        queens: Bitboard(0),
        kings: Bitboard(0),
        white: Bitboard(0),
        black: Bitboard(0),
    };
    pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
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
                    board.pawns |= Bitboard::from_square([file, rank]);
                    board.white |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'N' => {
                    board.knights |= Bitboard::from_square([file, rank]);
                    board.white |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'B' => {
                    board.bishops |= Bitboard::from_square([file, rank]);
                    board.white |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'R' => {
                    board.rooks |= Bitboard::from_square([file, rank]);
                    board.white |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'Q' => {
                    board.queens |= Bitboard::from_square([file, rank]);
                    board.white |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'K' => {
                    board.kings |= Bitboard::from_square([file, rank]);
                    board.white |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'p' => {
                    board.pawns |= Bitboard::from_square([file, rank]);
                    board.black |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'n' => {
                    board.knights |= Bitboard::from_square([file, rank]);
                    board.black |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'b' => {
                    board.bishops |= Bitboard::from_square([file, rank]);
                    board.black |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'r' => {
                    board.rooks |= Bitboard::from_square([file, rank]);
                    board.black |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'q' => {
                    board.queens |= Bitboard::from_square([file, rank]);
                    board.black |= Bitboard::from_square([file, rank]);
                    file += 1;
                }
                'k' => {
                    board.kings |= Bitboard::from_square([file, rank]);
                    board.black |= Bitboard::from_square([file, rank]);
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

    pub fn get_color(&self, square: Bitboard) -> Option<Color> {
        if self.white & square != Bitboard(0) {
            Some(Color::White)
        } else if self.black & square != Bitboard(0) {
            Some(Color::Black)
        } else {
            None
        }
    }

    pub fn is_occupied(&self) -> Bitboard {
        self.pawns
            | self.knights
            | self.bishops
            | self.rooks
            | self.queens
            | self.kings
            | self.white
            | self.black
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Bitboard::from_square([file, rank]);
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
        write!(f, "{:18x} -> {:18x}", self.from.0, self.to.0)
    }
}

// pseudo-legal moves
// Does not check for check or pinned pieces
pub fn gen_moves(board: &Board, origin_square: Bitboard) -> Vec<Move> {
    let piece = match board.get_piece(origin_square) {
        Some(piece) => piece,
        None => return vec![],
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
            KNIGHT_MOVES.iter().for_each(|&(dx, dy)| {
                let to = origin_square ^ Bitboard::from_square([dx as u8, dy as u8]);
                moves.push(Move::new(origin_square, to));
            });
            moves
        }
        PieceKind::Bishop => {
            let mut moves: Vec<Move> = vec![];
            // TODO: raycast moves
            moves
        }
        PieceKind::Rook => {
            let mut moves: Vec<Move> = vec![];
            // TODO: raycast moves
            moves
        }
        PieceKind::Queen => {
            let mut moves: Vec<Move> = vec![];
            // TODO: raycast moves
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
            .for_each(|&to| moves.push(Move::new(origin_square, to)));
            moves
        }
    };
    moves
}

pub fn make_move(board: &mut Board, mov: Move) {}
