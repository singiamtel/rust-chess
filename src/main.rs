use board::gen_moves;

use crate::{board::Board, piece::PieceKind};

mod board;
mod eval;
mod game;
mod movement;
mod perft;
mod piece;
mod printer;
mod search;
mod uci;

fn main() {
    let board = Board::new(Board::STARTING_FEN);
    // println!("{}", board);
    // let coords = [4, 0];
    // let moves: Vec<Move> = gen_moves_from_piece(&board, Bitboard::FROM_SQUARE(coords));
    // let piece = match board.get_piece(Bitboard::FROM_SQUARE(coords)) {
    //     Some(p) => p,
    //     None => panic!("No piece found"),
    // };
    let moves = gen_moves(&board);
    // count how many moves are generated per piece
    let mut move_count = 0;
    let mut pawn_moves = 0;
    let mut knight_moves = 0;
    let mut bishop_moves = 0;
    let mut rook_moves = 0;
    let mut queen_moves = 0;
    let mut king_moves = 0;
    for m in moves.iter() {
        move_count += 1;
        match board.get_piece(m.from).unwrap().kind {
            PieceKind::Pawn => pawn_moves += 1,
            PieceKind::Knight => knight_moves += 1,
            PieceKind::Bishop => bishop_moves += 1,
            PieceKind::Rook => rook_moves += 1,
            PieceKind::Queen => queen_moves += 1,
            PieceKind::King => king_moves += 1,
        };
    }
    println!("Total legal moves in turn 1: {}", move_count);
    println!("Pawn: {}", pawn_moves);
    println!("Knight: {}", knight_moves);
    println!("Bishop: {}", bishop_moves);
    println!("Rook: {}", rook_moves);
    println!("Queen: {}", queen_moves);
    println!("King: {}", king_moves);
}
