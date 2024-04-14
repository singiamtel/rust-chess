use crate::{bitboard::Bitboard, board::Board, piece::Color};

impl Board {
    const DEFAULT: Self = Self {
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

    pub const STARTING_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub fn new(fen: &str) -> Self {
        let mut board = Self::DEFAULT;
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
                    panic!("Invalid FEN character: {c}");
                }
            }
        }
        board
    }
}
