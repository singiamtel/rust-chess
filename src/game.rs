use crate::{
    bitboard::Bitboard,
    board::Board,
    piece::{Color, Piece, PieceKind},
    r#move::{bitboard_to_algebraic, Move},
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
}

impl Game {
    const DEFAULT: Self = Self {
        board: Board::DEFAULT,
        history: vec![],
        en_passant: None,
        castling: 0,
        halfmove_clock: 0,
        fullmove_number: 1,
        turn: Color::White,
    };

    pub const STARTING_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub fn new(fen: &str) -> Self {
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
                    game.board.pawns |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'N' => {
                    game.board.knights |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'B' => {
                    game.board.bishops |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'R' => {
                    game.board.rooks |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'Q' => {
                    game.board.queens |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'K' => {
                    game.board.kings |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.white |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'p' => {
                    game.board.pawns |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'n' => {
                    game.board.knights |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'b' => {
                    game.board.bishops |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'r' => {
                    game.board.rooks |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'q' => {
                    game.board.queens |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                'k' => {
                    game.board.kings |= Bitboard::FROM_SQUARE([file, rank]);
                    game.board.black |= Bitboard::FROM_SQUARE([file, rank]);
                    file += 1;
                }
                '1'..='8' => file += c as u8 - b'0',
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                _ => {
                    panic!("Invalid FEN character: {c}");
                }
            }
        }
        game
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
    game: &Game,
    piece: &Piece,
    origin_square: Bitboard,
    direction: &Direction,
) {
    let (color_mask, opposite_color_mask) = if piece.color == Color::White {
        (game.board.white, game.board.black)
    } else {
        (game.board.black, game.board.white)
    };
    let to = match direction {
        Direction::North => origin_square.north(),
        Direction::South => origin_square.south(),
        Direction::East => origin_square.east(),
        Direction::West => origin_square.west(),
        Direction::NorthEast => origin_square.north_east(),
        Direction::NorthWest => origin_square.north_west(),
        Direction::SouthEast => origin_square.south_east(),
        Direction::SouthWest => origin_square.south_west(),
    };

    if !to.is_empty() && !to.intersects(color_mask) {
        let mut new_move = Move::new(origin_square, to, *piece);
        // check if it's a capture
        if to.intersects(opposite_color_mask) {
            new_move = new_move.with_capture(game.board.get_piece(to).unwrap());
        }
        moves.push(new_move);
    }
}

// pseudo-legal moves
// Does not check for check or pinned pieces
pub fn gen_moves_from_piece(game: &Game, origin_square: Bitboard) -> Vec<Move> {
    let Some(piece) = game.board.get_piece(origin_square) else {
        return vec![];
    };
    let (current_turn_mask, opposite_color_mask) = if game.turn == Color::White {
        (game.board.white, game.board.black)
    } else {
        (game.board.black, game.board.white)
    };
    let moves: Vec<Move> = match piece.kind {
        PieceKind::Pawn => {
            let mut moves: Vec<Move> = vec![];
            let to: Bitboard = if game.turn == Color::White {
                origin_square.north()
            } else {
                origin_square.south()
            };
            let colors_mask = game.board.white | game.board.black;
            if !to.is_empty() && !to.intersects(colors_mask) {
                // is promotion?
                let new_move = Move::new(origin_square, to, piece);
                if origin_square.intersects(Bitboard::RANK_1 | Bitboard::RANK_8) {
                    moves.append(&mut new_move.with_promotions());
                } else {
                    moves.push(new_move);
                }

                if origin_square.pawn_initial(current_turn_mask) {
                    let to = if game.turn == Color::White {
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
                if game.turn == Color::White && !origin_square.intersects(Bitboard::FILE_A) {
                    origin_square.north_east()
                } else {
                    origin_square.south_east()
                },
                if game.turn == Color::White && !origin_square.intersects(Bitboard::FILE_H) {
                    origin_square.north_west()
                } else {
                    origin_square.south_west()
                },
            ] {
                if !to.is_empty() && to.intersects(opposite_color_mask) {
                    moves.push(
                        Move::new(origin_square, to, piece)
                            .with_capture(game.board.get_piece(to).unwrap()),
                    );
                }
            }
            // TODO: capture moves, en passant
            moves
        }
        PieceKind::Knight => {
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
                    println!(
                        "knight from: {} to: {}",
                        bitboard_to_algebraic(origin_square),
                        bitboard_to_algebraic(to)
                    );
                    let mut new_move = Move::new(origin_square, to, piece);
                    if to.intersects(opposite_color_mask) {
                        new_move = new_move.with_capture(game.board.get_piece(to).unwrap());
                    }
                    moves.push(new_move);
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
                gen_sliding_moves(&mut moves, game, &piece, origin_square, &direction);
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
                gen_sliding_moves(&mut moves, game, &piece, origin_square, &direction);
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
                gen_sliding_moves(&mut moves, game, &piece, origin_square, direction);
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
            .for_each(|&to| moves.push(Move::new(origin_square, to, piece)));
            moves
        }
    };
    moves
}

pub fn gen_moves(game: &Game) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let occupied = game.board.occupied();

    let current_turn_mask = if game.turn == Color::White {
        game.board.white
    } else {
        game.board.black
    };
    for i in 0..64 {
        let square = Bitboard(1 << i);
        if occupied & square & current_turn_mask != Bitboard(0) {
            #[cfg(debug_assertions)]
            {
                game.board
                    .get_piece(square)
                    .map_or_else(|| panic!("No piece found at square: {i}"), |piece| piece);
            }
            let mut piece_moves = gen_moves_from_piece(game, square);
            moves.append(&mut piece_moves);
        }
    }

    moves.into_iter().filter(|b| !b.to.is_empty()).collect()
}

pub fn make_move(game: &mut Game, mov: Move) {
    game.board.make_move(mov);
    game.turn = game.turn.opposite();
    game.history.push(mov);
    game.fullmove_number += 1;
    game.halfmove_clock += 1;
}

pub fn unmake_move(game: &mut Game, mov: Move) {
    // let mov = game.history.pop().expect("No moves to undo");
    game.history.pop().expect("No moves to undo");
    game.board.unmove_piece(mov);
    game.turn = game.turn.opposite();
    // restore old piece
    if let Some(captured_piece) = mov.capture {
        println!("spawning captured piece: {captured_piece}");
        game.board.spawn_piece(captured_piece, mov.to);
    }
    game.fullmove_number -= 1;
    game.halfmove_clock -= 1;
}
