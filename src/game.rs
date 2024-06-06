use std::error::Error;

use crate::history::HistoryItem;
use crate::move_generation::Movegen;
use crate::{
    bitboard::{display::BitboardDisplay, Bitboard, BitboardError},
    board::{Board, CastlingRights},
    history::History,
    move_generation::error::MovegenError,
    piece::{Color, Kind, Piece},
    r#move::Move,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    pub is_in_check: bool,
    pub history: History,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenError {
    InvalidFen(String, char),
    InvalidEnPassant(String),
}

impl From<BitboardError> for FenError {
    fn from(err: BitboardError) -> Self {
        Self::InvalidEnPassant(err.to_string())
    }
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidFen(fen, c) => {
                write!(f, "Invalid FEN string: {fen}, invalid character: {c}")
            }
            Self::InvalidEnPassant(en_passant) => {
                write!(
                    f,
                    "Invalid FEN string: {en_passant}, invalid en passant square"
                )
            }
        }
    }
}
impl Error for FenError {}

impl Game {
    pub const STARTING_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub fn new(fen: &str) -> Result<Self, FenError> {
        let mut board = Board::new();
        let mut rank = 7;
        let mut file = 0;
        let splitted_vec = fen.split(' ').collect::<Vec<&str>>();
        assert!(splitted_vec.len() >= 4); // halfmove clock, fullmove number can be omitted
        let mut splitted_iter = splitted_vec.into_iter();
        let pieces = splitted_iter.next().map_or_else(
            || {
                panic!("Invalid FEN string: {fen}");
            },
            |pieces| pieces,
        );

        for c in pieces.chars() {
            match c {
                'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                    board.spawn_piece(Piece::new(
                        Color::White,
                        match c {
                            'P' => Kind::Pawn,
                            'N' => Kind::Knight,
                            'B' => Kind::Bishop,
                            'R' => Kind::Rook,
                            'Q' => Kind::Queen,
                            'K' => Kind::King,
                            _ => unreachable!(),
                        },
                        Bitboard::from_square(file, rank),
                    ));
                    file += 1;
                }
                'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                    board.spawn_piece(Piece::new(
                        Color::Black,
                        match c {
                            'p' => Kind::Pawn,
                            'n' => Kind::Knight,
                            'b' => Kind::Bishop,
                            'r' => Kind::Rook,
                            'q' => Kind::Queen,
                            'k' => Kind::King,
                            _ => unreachable!(),
                        },
                        Bitboard::from_square(file, rank),
                    ));
                    file += 1;
                }
                '1'..='8' => file += c as u8 - b'0',
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                _ => {
                    return Err(FenError::InvalidFen(fen.to_string(), c));
                }
            }
        }

        let turn = match splitted_iter.next().unwrap() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => {
                panic!("Invalid FEN string: {fen}");
            }
        };
        board.turn = turn;

        let castling_rights = splitted_iter.next().unwrap();

        let mut set_castling_right =
            |right: CastlingRights| board.castling.set_castling_right(right, true);
        for c in castling_rights.chars() {
            match c {
                'K' => set_castling_right(CastlingRights::WHITE_KINGSIDE),
                'Q' => set_castling_right(CastlingRights::WHITE_QUEENSIDE),
                'k' => set_castling_right(CastlingRights::BLACK_KINGSIDE),
                'q' => set_castling_right(CastlingRights::BLACK_QUEENSIDE),
                '-' => (),
                _ => panic!("Invalid FEN string: {fen}"),
            }
        }

        let en_passant_str = splitted_iter.next().unwrap();

        board.en_passant = if en_passant_str == "-" {
            None
        } else {
            Some(Bitboard::from_algebraic(en_passant_str)?)
        };

        let halfmove_clock = match splitted_iter.next() {
            Some(halfmove_clock) => halfmove_clock.parse().unwrap(),
            None => 0,
        };

        let fullmove_number = match splitted_iter.next() {
            Some(fullmove_number) => fullmove_number.parse().unwrap(),
            None => 1,
        };

        Ok(Game {
            board,
            history: History(vec![]),
            is_in_check: false,
            halfmove_clock,
            fullmove_number,
        })
    }

    pub fn make_move(&mut self, mov: Move) {
        self.board.move_piece(mov);

        self.history.push(HistoryItem {
            r#move: mov,
            squares_attacked: self.board.attacked_squares,
        });
        self.fullmove_number += 1;
        self.halfmove_clock += 1;
        self.is_in_check = self.board.is_check(self.board.turn);

        if self.is_in_check {
            // remove castling rights to the color in check
            // println!("{} is in check, removing castling rights ({})", self.turn, mov);
            match !self.board.turn {
                Color::White => self
                    .board
                    .castling
                    .set_castling_right(CastlingRights::WHITE_BOTH, false),
                Color::Black => self
                    .board
                    .castling
                    .set_castling_right(CastlingRights::BLACK_BOTH, false),
            }
        }

        self.board.flip_turn();
    }

    pub fn unmake_move(&mut self, mov: Move) {
        // let mov = game.history.pop().expect("No moves to undo");
        self.history.pop().expect("No moves to undo");
        self.board.unmove_piece(mov);
        self.board.flip_turn();
        self.fullmove_number -= 1;
        self.halfmove_clock -= 1;
    }

    pub fn parse_move(&self, r#move: &str) -> Result<Move, MovegenError> {
        // println!("Parsing move: {}", r#move);
        let from = Bitboard::from_algebraic(&r#move[0..2])?;
        let to = Bitboard::from_algebraic(&r#move[2..4])?;
        let legal_moves = self.board.gen_moves()?;
        for legal_move in legal_moves {
            if legal_move.from == from && legal_move.to == to {
                return Ok(legal_move);
            }
        }
        Err(MovegenError::InvalidMove(r#move.to_string()))
    }
}
