use std::error::Error;

use crate::{
    bitboard::{
        display::BitboardDisplay, generate_knight_lookup, generate_pawn_lookup, Bitboard,
        BitboardError, Direction, DirectionalShift,
    },
    board::{Board, CastlingRights, OnePerColor},
    piece::{Color, Kind, Piece},
    r#move::Move,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct History(Vec<Move>);

impl std::fmt::Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // print all moves in algebraic notation
        for r#move in &self.0 {
            let _ = write!(f, "{move} ");
        }
        Ok(())
    }
}

impl History {
    pub fn push(&mut self, r#move: Move) {
        self.0.push(r#move);
    }
    pub fn pop(&mut self) -> Option<Move> {
        self.0.pop()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovegenError {
    InvalidMove(String),
    BitboardError(BitboardError),
}

impl From<Move> for MovegenError {
    fn from(r#move: Move) -> Self {
        Self::InvalidMove(r#move.to_string())
    }
}

impl std::fmt::Display for MovegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidMove(r#move) => write!(f, "Invalid move: {}", r#move),
            Self::BitboardError(err) => write!(f, "Bitboard error: {}", err),
        }
    }
}

impl From<BitboardError> for MovegenError {
    fn from(err: BitboardError) -> Self {
        Self::BitboardError(err)
    }
}

impl Error for MovegenError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub history: History,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub pawn_attacks_lookup: OnePerColor<[Bitboard; 64]>,
    pub knight_attacks_lookup: [Bitboard; 64],
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
        let mut board = Board::DEFAULT;
        let mut rank = 7;
        let mut file = 0;
        let splitted_vec = fen.split(' ').collect::<Vec<&str>>();
        assert_eq!(splitted_vec.len(), 6);
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

        let castling_rights = splitted_iter.next().unwrap();

        let mut set_castling_right = |right| board.castling.set_castling_right(right, true);
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

        let _pawn_attacks_lookup = generate_pawn_lookup();
        let knight_attacks_lookup = generate_knight_lookup();
        let pawn_attacks_lookup =
            OnePerColor::new(_pawn_attacks_lookup[0], _pawn_attacks_lookup[1]);
        Ok(Game {
            board,
            turn,
            history: History(vec![]),
            halfmove_clock: 0,  // TODO: implement
            fullmove_number: 1, // TODO: implement
            pawn_attacks_lookup,
            knight_attacks_lookup,
        })
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
        let to = current_square.shift(direction);

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
                        let new_to = if self.turn == Color::White {
                            origin_square.north().north()
                        } else {
                            origin_square.south().south()
                        };

                        if new_to != Bitboard(0) && (new_to & colors_mask) == Bitboard(0) {
                            let mov = Move::new(origin_square, new_to, piece).with_en_passant(to);
                            // println!("Move vulnerable to en passant: {} {}", mov, to.to_algebraic().unwrap());
                            moves.push(mov);
                        }
                    }
                }
                // captures
                // println!(
                //     "Generating pawn captures for {} at {}",
                //     piece,
                //     origin_square.to_algebraic().unwrap()
                // );
                // if let Some(en_passant_square) = self.board.en_passant {
                //     println!(
                //         "Current en passant: {}",
                //         en_passant_square.to_algebraic().unwrap()
                //     );
                // }
                for to in Direction::pawn_captures(self.turn) {
                    let to = origin_square.shift(&to);
                    if to.is_empty() {
                        continue;
                    }

                    // Regular capture
                    if to.intersects(opposite_color_mask) {
                        let new_move = Move::new(origin_square, to, piece)
                            .with_capture(self.board.get_piece(to).unwrap());
                        if to.intersects(Bitboard::PAWN_PROMOTION_MASK) {
                            moves.append(&mut new_move.with_promotions());
                        } else {
                            moves.push(new_move);
                        }
                    } else if let Some(en_passant_square) = self.board.en_passant {
                        if to == en_passant_square {
                            let victim_pawn = self
                                .board
                                .get_en_passant_victim(en_passant_square, !self.turn);

                            let new_move =
                                Move::new(origin_square, to, piece).with_capture(victim_pawn);
                            moves.push(new_move);
                        }
                    }
                }
                moves
            }
            Kind::Knight => {
                let mut moves: Vec<Move> = vec![];

                for &knight_move in &Direction::KNIGHT_MOVES {
                    let to = origin_square.shift(&knight_move);
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
                for direction in &Direction::DIAGONAL_MOVES {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, direction);
                }
                moves
            }
            Kind::Rook => {
                let mut moves: Vec<Move> = vec![];
                for direction in &Direction::STRAIGHT_MOVES {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, direction);
                }
                moves
            }
            Kind::Queen => {
                let mut moves: Vec<Move> = vec![];
                for direction in &Direction::SLIDING_MOVES {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, direction);
                }
                moves
            }
            // TODO: implement castling
            Kind::King => {
                let mut moves: Vec<Move> = vec![];
                for direction in &Direction::SLIDING_MOVES {
                    let to = origin_square.shift(direction);
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
        };
        moves
    }

    fn slide_until_blocked(
        &self,
        current_square: Bitboard,
        direction: &Direction,
        color: Color,
    ) -> Option<Piece> {
        let (color_mask, opposite_color_mask) = if color == Color::White {
            (self.board.white, self.board.black)
        } else {
            (self.board.black, self.board.white)
        };
        let to = current_square.shift(direction);

        if to.is_empty() {
            None
        } else {
            // if its evil piece
            if to.intersects(opposite_color_mask) {
                Some(self.board.get_piece(to).unwrap())
            }
            // if its friendly piece
            else if to.intersects(color_mask) {
                None
            } else {
                self.slide_until_blocked(to, direction, color)
            }
        }
    }

    fn king_position(&self, color: Color) -> usize {
        match color {
            Color::White => self
                .board
                .king_position
                .white
                .expect("King position not set"),
            Color::Black => self
                .board
                .king_position
                .black
                .expect("King position not set"),
        }
    }

    pub fn is_check(&mut self) -> bool {
        let color = !self.turn; // We want to check if the last move was a self-check
        let king_position = self.king_position(color);
        let (color_mask, opposite_color_mask) = if color == Color::White {
            (self.board.white, self.board.black)
        } else {
            (self.board.black, self.board.white)
        };
        if (self.pawn_attacks_lookup.get(!color)[king_position] // get the other color lookup
            & self.board.pawns
            & opposite_color_mask)
            != Bitboard(0)
        {
            return true;
        }
        // println!("History: {:}", self.history);
        // println!("Side checked: {}", color);
        // println!("Kings: {:#016x}", self.board.kings);
        // println!("King position: {}", Bitboard(1 << king_position).to_algebraic().unwrap());
        // println!("Knight attacks: {:#016x}", self.knight_attacks_lookup[king_position]);
        // println!("Knights: {:#016x}", self.board.knights & opposite_color_mask);
        if (self.knight_attacks_lookup[king_position] & (self.board.knights & opposite_color_mask))
            != Bitboard(0)
        {
            // println!("Knight check!");
            // // print all previous moves
            return true;
        }

        // TODO: Use magic bitboards and pre-computed lookup tables for sliding pieces
        for direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            // self.gen_sliding_moves(&mut moves, piece, origin_square, &direction);
            let piece = self.slide_until_blocked(self.board.kings & color_mask, &direction, color);
            if let Some(piece) = piece {
                match piece.kind {
                    Kind::Queen | Kind::Rook => {
                        // println!("Queen or Rook check!");
                        // println!("{:#016x}", opposite_color_mask);
                        // println!("{:#016x}", self.board.kings & color_mask);
                        // println!("{}", piece);
                        return true;
                    }
                    _ => {}
                }
            }
        }
        for direction in [
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ] {
            let piece = self.slide_until_blocked(self.board.kings & color_mask, &direction, color);
            if let Some(piece) = piece {
                match piece.kind {
                    Kind::Queen | Kind::Bishop => {
                        // println!("Queen or Bishop check!");
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    pub fn gen_moves(&self) -> Vec<Move> {
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
            self.board.spawn_piece(captured_piece);
        }
        self.fullmove_number -= 1;
        self.halfmove_clock -= 1;
    }

    pub fn parse_move(&self, r#move: &str) -> Result<Move, MovegenError> {
        // println!("Parsing move: {}", r#move);
        let from = Bitboard::from_algebraic(&r#move[0..2])?;
        let to = Bitboard::from_algebraic(&r#move[2..4])?;
        let legal_moves = self.gen_moves();
        for legal_move in legal_moves {
            if legal_move.from == from && legal_move.to == to {
                return Ok(legal_move);
            }
        }
        Err(MovegenError::InvalidMove(r#move.to_string()))
    }
}
