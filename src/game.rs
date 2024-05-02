use crate::{
    bitboard::Bitboard,
    board::Board,
    piece::{Color, Piece, PieceKind},
    r#move::Move,
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

const KNIGHT_MOVES: [u8; 4] = [17, 15, 10, 6];

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
    let current_turn_mask = if piece.color == Color::White {
        game.board.white
    } else {
        game.board.black
    };
    match direction {
        Direction::North => {
            let to = origin_square.north();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
        Direction::South => {
            let to = origin_square.south();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
        Direction::East => {
            let to = origin_square.east();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
        Direction::West => {
            let to = origin_square.west();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
        Direction::NorthEast => {
            let to = origin_square.north_east();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
        Direction::NorthWest => {
            let to = origin_square.north_west();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
        Direction::SouthEast => {
            let to = origin_square.south_east();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
        Direction::SouthWest => {
            let to = origin_square.south_west();
            if to != Bitboard(0) && to & current_turn_mask == Bitboard(0) {
                moves.push(Move::new(origin_square, to, None));
            }
        }
    }
}

// pseudo-legal moves
// Does not check for check or pinned pieces
pub fn gen_moves_from_piece(game: &Game, origin_square: Bitboard) -> Vec<Move> {
    let piece = match game.board.get_piece(origin_square) {
        Some(piece) => piece,
        None => return vec![],
    };
    let current_turn_mask = if game.turn == Color::White {
        game.board.white
    } else {
        game.board.black
    };
    let moves: Vec<Move> = match piece.kind {
        PieceKind::Pawn => {
            let mut moves: Vec<Move> = vec![];
            let to: Bitboard = origin_square.north();
            moves.push(Move::new(origin_square, to, None));
            if origin_square.pawn_initial(current_turn_mask) {
                moves.push(Move::new(origin_square, to.north(), None));
            }
            // TODO: capture moves, en passant, and promotion
            moves
        }
        PieceKind::Knight => {
            let mut moves: Vec<Move> = vec![];
            for &offset in &KNIGHT_MOVES {
                let positive = origin_square << offset.into();
                if positive & current_turn_mask != Bitboard(0) {
                    moves.push(Move::new(origin_square, positive, None));
                }
                let negative = origin_square >> offset.into();
                if negative & current_turn_mask != Bitboard(0) {
                    moves.push(Move::new(origin_square, negative, None));
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
            .for_each(|&to| moves.push(Move::new(origin_square, to, None)));
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
                match game.board.get_piece(square) {
                    Some(piece) => piece,
                    None => panic!("No piece found at square: {i}"),
                };
            }
            let mut piece_moves = gen_moves_from_piece(game, square);
            moves.append(&mut piece_moves);
        }
    }

    moves
}
