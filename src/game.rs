use std::error::Error;

use crate::{
    bitboard::{generate_pawn_lookup, Bitboard, Direction, DirectionalShift},
    board::Board,
    piece::{Color, Kind, Piece},
    r#move::{algebraic_to_bitboard, BitboardError, Move},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub history: Vec<Move>,

    pub en_passant: Option<Bitboard>,
    pub castling: u8,

    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub pawn_attacks_lookup: Option<[[Bitboard; 64]; 2]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenError {
    InvalidFen(String, char),
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidFen(fen, c) => {
                write!(f, "Invalid FEN string: {fen}, invalid character: {c}")
            }
        }
    }
}
impl Error for FenError {}

impl Game {
    const DEFAULT: Self = Self {
        board: Board::DEFAULT,
        history: vec![],
        en_passant: None,
        castling: 0,
        halfmove_clock: 0,
        fullmove_number: 1,
        turn: Color::White,
        pawn_attacks_lookup: None,
    };

    pub const STARTING_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub fn new(fen: &str) -> Result<Self, FenError> {
        let mut game = Self::DEFAULT;
        let mut rank = 7;
        let mut file = 0;
        let splitted: Vec<&str> = fen.split(' ').collect();
        assert_eq!(splitted.len(), 6);
        let pieces = splitted.first().map_or_else(
            || {
                panic!("Invalid FEN string: {fen}");
            },
            |pieces| pieces,
        );

        for c in pieces.chars() {
            match c {
                'P' => {
                    game.board.spawn_piece(
                        Piece::new(Color::White, Kind::Pawn),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'N' => {
                    game.board.spawn_piece(
                        Piece::new(Color::White, Kind::Knight),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'B' => {
                    game.board.spawn_piece(
                        Piece::new(Color::White, Kind::Bishop),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'R' => {
                    game.board.spawn_piece(
                        Piece::new(Color::White, Kind::Rook),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'Q' => {
                    game.board.spawn_piece(
                        Piece::new(Color::White, Kind::Queen),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'K' => {
                    game.board.spawn_piece(
                        Piece::new(Color::White, Kind::King),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'p' => {
                    game.board.spawn_piece(
                        Piece::new(Color::Black, Kind::Pawn),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'n' => {
                    game.board.spawn_piece(
                        Piece::new(Color::Black, Kind::Knight),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'b' => {
                    game.board.spawn_piece(
                        Piece::new(Color::Black, Kind::Bishop),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'r' => {
                    game.board.spawn_piece(
                        Piece::new(Color::Black, Kind::Rook),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'q' => {
                    game.board.spawn_piece(
                        Piece::new(Color::Black, Kind::Queen),
                        Bitboard::from_square(file, rank),
                    );
                    file += 1;
                }
                'k' => {
                    game.board.spawn_piece(
                        Piece::new(Color::Black, Kind::King),
                        Bitboard::from_square(file, rank),
                    );
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
        game.pawn_attacks_lookup = Some(generate_pawn_lookup());
        Ok(game)
    }

    fn __gen_sliding_moves_recursive(
        &self,
        moves: &mut Vec<Move>,
        piece: Piece,
        origin_square: Bitboard,
        current_square: Bitboard,
        direction: &Direction,
    ) {
        let (color_mask, opposite_color_mask) = if piece.color == Color::White {
            (self.board.white, self.board.black)
        } else {
            (self.board.black, self.board.white)
        };
        let to = match direction {
            Direction::North => current_square.north(),
            Direction::South => current_square.south(),
            Direction::East => current_square.east(),
            Direction::West => current_square.west(),
            Direction::NorthEast => current_square.north_east(),
            Direction::NorthWest => current_square.north_west(),
            Direction::SouthEast => current_square.south_east(),
            Direction::SouthWest => current_square.south_west(),
        };

        if !to.is_empty() && !to.intersects(color_mask) {
            let mut new_move = Move::new(origin_square, to, piece);
            // check if it's a capture
            if to.intersects(opposite_color_mask) {
                new_move = new_move.with_capture(self.board.get_piece(to).unwrap());
                moves.push(new_move);
            } else {
                moves.push(new_move);
                self.__gen_sliding_moves_recursive(moves, piece, origin_square, to, direction);
            }
        }
    }

    pub fn gen_sliding_moves(
        &self,
        moves: &mut Vec<Move>,
        piece: Piece,
        origin_square: Bitboard,
        direction: &Direction,
    ) {
        self.__gen_sliding_moves_recursive(moves, piece, origin_square, origin_square, direction);
    }

    // pseudo-legal moves
    // Does not check for check or pinned pieces
    pub fn gen_moves_from_piece(&self, origin_square: Bitboard) -> Vec<Move> {
        let Some(piece) = self.board.get_piece(origin_square) else {
            return vec![];
        };
        let (current_turn_mask, opposite_color_mask) = if self.turn == Color::White {
            (self.board.white, self.board.black)
        } else {
            (self.board.black, self.board.white)
        };
        let moves: Vec<Move> = match piece.kind {
            Kind::Pawn => {
                let mut moves: Vec<Move> = vec![];
                let to: Bitboard = if self.turn == Color::White {
                    origin_square.north()
                } else {
                    origin_square.south()
                };
                let colors_mask = self.board.white | self.board.black;
                if !to.is_empty() && !to.intersects(colors_mask) {
                    // is promotion?
                    let new_move = Move::new(origin_square, to, piece);
                    if to.intersects(Bitboard::RANK_1 | Bitboard::RANK_8) {
                        moves.append(&mut new_move.with_promotions());
                    } else {
                        moves.push(new_move);
                    }

                    if origin_square.pawn_initial(current_turn_mask) {
                        let to = if self.turn == Color::White {
                            origin_square.north().north()
                        } else {
                            origin_square.south().south()
                        };

                        if to != Bitboard(0) && to & colors_mask == Bitboard(0) {
                            moves.push(Move::new(origin_square, to, piece));
                        }
                    }
                }
                // captures
                for to in [
                    if self.turn == Color::White && !origin_square.intersects(Bitboard::FILE_H) {
                        origin_square.north_east()
                    } else {
                        origin_square.south_east()
                    },
                    if self.turn == Color::White && !origin_square.intersects(Bitboard::FILE_A) {
                        origin_square.north_west()
                    } else {
                        origin_square.south_west()
                    },
                ] {
                    if !to.is_empty() && to.intersects(opposite_color_mask) {
                        let new_move = Move::new(origin_square, to, piece)
                            .with_capture(self.board.get_piece(to).unwrap());
                        if to.intersects(Bitboard::RANK_1 | Bitboard::RANK_8) {
                            moves.append(&mut new_move.with_promotions());
                        } else {
                            moves.push(new_move);
                        }
                    }
                }
                // TODO: en passant
                moves
            }
            Kind::Knight => {
                let mut moves: Vec<Move> = vec![];
                for to in [
                    origin_square.no_no_ea(),
                    origin_square.no_ea_ea(),
                    origin_square.so_ea_ea(),
                    origin_square.so_so_ea(),
                    origin_square.no_no_we(),
                    origin_square.no_we_we(),
                    origin_square.so_we_we(),
                    origin_square.so_so_we(),
                ] {
                    if !to.is_empty() && !to.intersects(current_turn_mask) {
                        let mut new_move = Move::new(origin_square, to, piece);
                        if to.intersects(opposite_color_mask) {
                            new_move = new_move.with_capture(self.board.get_piece(to).unwrap());
                        }
                        moves.push(new_move);
                    }
                }
                moves
            }
            Kind::Bishop => {
                let mut moves: Vec<Move> = vec![];
                for direction in [
                    Direction::NorthEast,
                    Direction::NorthWest,
                    Direction::SouthEast,
                    Direction::SouthWest,
                ] {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, &direction);
                }
                moves
            }
            Kind::Rook => {
                let mut moves: Vec<Move> = vec![];
                for direction in [
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ] {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, &direction);
                }
                moves
            }
            Kind::Queen => {
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
                    self.gen_sliding_moves(&mut moves, piece, origin_square, direction);
                }
                moves
            }
            Kind::King => {
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
                .for_each(|&to| moves.push(Move::new(origin_square, to, piece)));
                moves
            }
        };
        moves
    }

    pub fn is_check(&mut self, color: Color) -> bool {
        let king = self.board.kings & self.board.get_color_mask(color);
        let mut moves: Vec<Move> = vec![];
        self.turn = !self.turn;
        moves.append(&mut self.gen_moves());
        // let ret = moves.clone().into_iter().any(|m| m.to == king);
        let ret = false; // TODO: Implement
        if ret {
            println!("Check! by: {:?}", moves.into_iter().find(|m| m.to == king));
        }
        self.turn = !self.turn;
        ret
    }

    pub fn gen_moves(&mut self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let occupied = self.board.occupied();

        let current_turn_mask = if self.turn == Color::White {
            self.board.white
        } else {
            self.board.black
        };
        for i in 0..64 {
            let square = Bitboard(1 << i);
            if occupied & square & current_turn_mask != Bitboard(0) {
                #[cfg(debug_assertions)]
                {
                    self.board
                        .get_piece(square)
                        .map_or_else(|| panic!("No piece found at square: {i}"), |piece| piece);
                }
                let mut piece_moves = self.gen_moves_from_piece(square);
                moves.append(&mut piece_moves);
            }
        }

        moves.into_iter().filter(|b| !b.to.is_empty()).collect()
    }

    pub fn make_move(&mut self, mov: Move) {
        self.board.move_piece(mov);
        self.turn = !self.turn;
        self.history.push(mov);
        self.fullmove_number += 1;
        self.halfmove_clock += 1;
    }

    pub fn unmake_move(&mut self, mov: Move) {
        // let mov = game.history.pop().expect("No moves to undo");
        self.history.pop().expect("No moves to undo");
        self.board.unmove_piece(mov);
        self.turn = !self.turn;
        // restore old piece
        if let Some(captured_piece) = mov.capture {
            self.board.spawn_piece(captured_piece, mov.to);
        }
        self.fullmove_number -= 1;
        self.halfmove_clock -= 1;
    }

    pub fn parse_move(&self, r#move: &str) -> Result<Move, BitboardError> {
        let from = algebraic_to_bitboard(&r#move[0..2])?;
        let to = algebraic_to_bitboard(&r#move[2..4])?;
        let what = self
            .board
            .get_piece(from)
            .ok_or(BitboardError::NoPieceAtSquare(from))?;
        Ok(Move::new(from, to, what))
    }
}
