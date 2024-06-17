use crate::{
    bitboard::{display::BitboardDisplay, Bitboard, Direction},
    board::{Board, CastlingRights},
    piece::{Color, Kind, Piece},
    r#move::Move,
};

use crate::bitboard::DirectionalShift;

use super::error::MovegenError;

pub trait Movegen {
    fn gen_sliding_moves_recursive(
        &self,
        moves: &mut Vec<Move>,
        piece: Piece,
        origin_square: Bitboard,
        current_square: Bitboard,
        direction: Direction,
    );
    fn gen_moves(&self) -> Result<Vec<Move>, MovegenError>;
    fn gen_sliding_moves(
        &self,
        moves: &mut Vec<Move>,
        piece: Piece,
        origin_square: Bitboard,
        direction: Direction,
    );
    fn gen_moves_from_piece(&self, origin_square: Bitboard) -> Vec<Move>;
    fn slide_until_blocked(
        &self,
        current_square: Bitboard,
        direction: Direction,
        color: Color,
    ) -> Option<Piece>;
    fn is_attacked(&self, square: Bitboard, idx: usize, color: Color) -> bool;
    fn is_check(&mut self, color: Color) -> bool;
}

impl Movegen for Board {
    fn gen_sliding_moves_recursive(
        &self,
        moves: &mut Vec<Move>,
        piece: Piece,
        origin_square: Bitboard,
        current_square: Bitboard,
        direction: Direction,
    ) {
        let (color_mask, opposite_color_mask) = if piece.color == Color::White {
            (self.white, self.black)
        } else {
            (self.black, self.white)
        };
        let to = current_square.shift(direction);

        if !to.is_empty() && !to.intersects(color_mask) {
            let mut new_move = Move::new(origin_square, to, piece);
            // check if it's a capture
            if to.intersects(opposite_color_mask) {
                new_move = new_move.with_capture(self.get_piece(to).unwrap());
                moves.push(new_move);
            } else {
                moves.push(new_move);
                self.gen_sliding_moves_recursive(moves, piece, origin_square, to, direction);
            }
        }
    }

    fn gen_sliding_moves(
        &self,
        moves: &mut Vec<Move>,
        piece: Piece,
        origin_square: Bitboard,
        direction: Direction,
    ) {
        self.gen_sliding_moves_recursive(moves, piece, origin_square, origin_square, direction);
    }

    // pseudo-legal moves
    // Does not check for check or pinned pieces
    fn gen_moves_from_piece(&self, origin_square: Bitboard) -> Vec<Move> {
        let Some(piece) = self.get_piece(origin_square) else {
            return vec![];
        };
        let (current_turn_mask, opposite_color_mask) = if self.turn == Color::White {
            (self.white, self.black)
        } else {
            (self.black, self.white)
        };
        let moves: Vec<Move> = match piece.kind {
            Kind::Pawn => {
                let mut moves: Vec<Move> = vec![];
                let to: Bitboard = if self.turn == Color::White {
                    origin_square.north()
                } else {
                    origin_square.south()
                };
                let colors_mask = self.white | self.black;
                if !to.is_empty() && !to.intersects(colors_mask) {
                    // is promotion?
                    let new_move = Move::new(origin_square, to, piece);
                    if to.intersects(Bitboard::PAWN_PROMOTION_MASK) {
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

                        if !new_to.is_empty() && !new_to.intersects(colors_mask) {
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
                    let to = origin_square.shift(to);
                    if to.is_empty() {
                        continue;
                    }

                    // Regular capture
                    if to.intersects(opposite_color_mask) {
                        let new_move = Move::new(origin_square, to, piece)
                            .with_capture(self.get_piece(to).unwrap());
                        if to.intersects(Bitboard::PAWN_PROMOTION_MASK) {
                            moves.append(&mut new_move.with_promotions());
                        } else {
                            moves.push(new_move);
                        }
                    } else if let Some(en_passant_square) = self.en_passant {
                        if to == en_passant_square {
                            let victim_pawn =
                                self.get_en_passant_victim(en_passant_square, !self.turn);

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
                    let to = origin_square.shift(knight_move);
                    if !to.is_empty() && !to.intersects(current_turn_mask) {
                        let mut new_move = Move::new(origin_square, to, piece);
                        if to.intersects(opposite_color_mask) {
                            new_move = new_move.with_capture(self.get_piece(to).unwrap());
                        }
                        moves.push(new_move);
                    }
                }

                moves
            }
            Kind::Bishop => {
                let mut moves: Vec<Move> = vec![];
                for direction in Direction::DIAGONAL_MOVES {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, direction);
                }
                moves
            }
            Kind::Rook => {
                let mut moves: Vec<Move> = vec![];
                for direction in Direction::STRAIGHT_MOVES {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, direction);
                }
                moves
            }
            Kind::Queen => {
                let mut moves: Vec<Move> = vec![];
                for direction in Direction::SLIDING_MOVES {
                    self.gen_sliding_moves(&mut moves, piece, origin_square, direction);
                }
                moves
            }
            Kind::King => {
                let mut moves: Vec<Move> = vec![];
                let lost_rights = match piece.color {
                    Color::White => CastlingRights::WHITE_BOTH,
                    Color::Black => CastlingRights::BLACK_BOTH,
                };
                for direction in Direction::SLIDING_MOVES {
                    let to = origin_square.shift(direction);
                    if !to.is_empty() && !to.intersects(current_turn_mask) {
                        let mut new_move = Move::new(origin_square, to, piece)
                            .with_castling_rights_loss(lost_rights);
                        if to.intersects(opposite_color_mask) {
                            new_move = new_move.with_capture(self.get_piece(to).unwrap());
                        }
                        moves.push(new_move);
                    }
                }
                // castling
                if origin_square.intersects(Bitboard::KING_INITIAL) {
                    // TODO: Long castle
                    match piece.color {
                        Color::White => {
                            // Short castle
                            if self
                                .castling
                                .get_castling_right(CastlingRights::WHITE_KINGSIDE)
                            {
                                let king_destination = origin_square.east().east();
                                let rook_origin = king_destination.east();
                                let rook_destination = origin_square.east();

                                // TODO: check if the king is in check during travel
                                if !rook_destination.intersects(self.anything())
                                    && !king_destination.intersects(self.anything())
                                    && !self.is_attacked(
                                        rook_destination,
                                        rook_destination.idx(),
                                        Color::White,
                                    )
                                {
                                    let mov = Move::new(origin_square, king_destination, piece)
                                        .with_castling_rights_loss(lost_rights)
                                        .with_castle_move((rook_origin, rook_destination));
                                    moves.push(mov);
                                }
                            }
                            // Long castle
                            if self
                                .castling
                                .get_castling_right(CastlingRights::WHITE_QUEENSIDE)
                            {
                                let travel_squares = [
                                    Bitboard::from_algebraic("b1").unwrap(),
                                    Bitboard::from_algebraic("c1").unwrap(),
                                    Bitboard::from_algebraic("d1").unwrap(),
                                ];

                                // only last two
                                let safe_squares = &travel_squares[1..];

                                let any_square_full = travel_squares
                                    .iter()
                                    .map(|square| square.intersects(self.anything()))
                                    .collect::<Vec<bool>>()
                                    .contains(&true);
                                let any_square_attacked = safe_squares
                                    .iter()
                                    .map(|square| {
                                        self.is_attacked(*square, square.idx(), Color::White)
                                    })
                                    .collect::<Vec<bool>>()
                                    .contains(&true);

                                if !any_square_attacked && !any_square_full {
                                    let mov = Move::new(
                                        origin_square,
                                        Bitboard::from_algebraic("c1").unwrap(),
                                        piece,
                                    )
                                    .with_castling_rights_loss(lost_rights)
                                    .with_castle_move((
                                        Bitboard::from_algebraic("a1").unwrap(),
                                        Bitboard::from_algebraic("d1").unwrap(),
                                    ));
                                    moves.push(mov);
                                }
                            }
                        }
                        Color::Black => {
                            // Short castle
                            if self
                                .castling
                                .get_castling_right(CastlingRights::BLACK_KINGSIDE)
                            {
                                let king_destination = origin_square.east().east();
                                let rook_origin = king_destination.east();
                                let rook_destination = origin_square.east();

                                // TODO: check if the king is in check during travel
                                if !rook_destination.intersects(self.anything())
                                    && !king_destination.intersects(self.anything())
                                    && !self.is_attacked(
                                        rook_destination,
                                        rook_destination.idx(),
                                        Color::Black,
                                    )
                                {
                                    let mov = Move::new(origin_square, king_destination, piece)
                                        .with_castling_rights_loss(lost_rights)
                                        .with_castle_move((rook_origin, rook_destination));
                                    moves.push(mov);
                                }
                            }

                            if self
                                .castling
                                .get_castling_right(CastlingRights::WHITE_QUEENSIDE)
                            {
                                let travel_squares = [
                                    Bitboard::from_algebraic("b8").unwrap(),
                                    Bitboard::from_algebraic("c8").unwrap(),
                                    Bitboard::from_algebraic("d8").unwrap(),
                                ];

                                // only last two
                                let safe_squares = &travel_squares[1..];

                                let any_square_full = travel_squares
                                    .iter()
                                    .map(|square| square.intersects(self.anything()))
                                    .collect::<Vec<bool>>()
                                    .contains(&true);
                                let any_square_attacked = safe_squares
                                    .iter()
                                    .map(|square| {
                                        self.is_attacked(*square, square.idx(), Color::Black)
                                    })
                                    .collect::<Vec<bool>>()
                                    .contains(&true);

                                if !any_square_attacked && !any_square_full {
                                    let mov = Move::new(
                                        origin_square,
                                        Bitboard::from_algebraic("c8").unwrap(),
                                        piece,
                                    )
                                    .with_castling_rights_loss(lost_rights)
                                    .with_castle_move((
                                        Bitboard::from_algebraic("a8").unwrap(),
                                        Bitboard::from_algebraic("d8").unwrap(),
                                    ));
                                    moves.push(mov);
                                }
                            }
                        }
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
        direction: Direction,
        color: Color,
    ) -> Option<Piece> {
        let (color_mask, opposite_color_mask) = if color == Color::White {
            (self.white, self.black)
        } else {
            (self.black, self.white)
        };
        let to = current_square.shift(direction);

        if to.is_empty() {
            None
        } else {
            // if its evil piece
            if to.intersects(opposite_color_mask) {
                Some(self.get_piece(to).unwrap())
            }
            // if its friendly piece
            else if to.intersects(color_mask) {
                None
            } else {
                self.slide_until_blocked(to, direction, color)
            }
        }
    }

    fn is_attacked(&self, square: Bitboard, idx: usize, color: Color) -> bool {
        // let color = !self.turn; // We want to check if the last move was a self-check
        // let (color_mask, opposite_color_mask) = if color == Color::White {
        //     (self.board.white, self.board.black)
        // } else {
        //     (self.board.black, self.board.white)
        // };
        let opposite_color_mask = self.get_color_mask(!color);
        if (self.pawn_attacks_lookup.get(!color)[idx] // get the other color lookup
            & self.pawns
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
        if (self.knight_attacks_lookup[idx] & (self.knights & opposite_color_mask)) != Bitboard(0) {
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
            let piece = self.slide_until_blocked(square, direction, color);
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
            let piece = self.slide_until_blocked(square, direction, color);
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

    fn is_check(&mut self, color: Color) -> bool {
        let king_position = self.king_position(color);
        let square = Bitboard(1 << king_position);
        #[cfg(debug_assertions)]
        {
            assert!(square.count() == 1);
        }
        self.is_attacked(square, king_position, color)
    }

    fn gen_moves(&self) -> Result<Vec<Move>, MovegenError> {
        let mut moves: Vec<Move> = vec![];

        let current_turn_mask = if self.turn == Color::White {
            self.white
        } else {
            self.black
        };
        for i in 0..64 {
            let square = Bitboard(1 << i);

            if square.intersects(current_turn_mask) {
                #[cfg(debug_assertions)]
                {
                    self.get_piece(square)
                        .map_or_else(|| panic!("No piece found at square: {i}"), |piece| piece);
                }
                let mut piece_moves = self.gen_moves_from_piece(square);
                moves.append(&mut piece_moves);
            }
        }

        Ok(moves.into_iter().filter(|b| !b.to.is_empty()).collect())
    }
}
