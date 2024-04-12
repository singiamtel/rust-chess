use crate::board::{gen_moves, Bitboard, Board, Move};

mod board;
mod eval;
mod game;
mod movement;
mod piece;
mod search;
mod uci;

fn main() {
    println!("Hello, world!");
    let board = Board::new(Board::STARTING_FEN);
    println!("{}", board);
    let coords = [4, 1];
    let moves: Vec<Move> = gen_moves(&board, Bitboard::from_square(coords));
    let piece = match board.get_piece(Bitboard::from_square(coords)) {
        Some(p) => p,
        None => panic!("No piece found"),
    };
    println!(
        "Moves for square({}):",
        board.get_piece(Bitboard::from_square(coords)).unwrap()
    );
    for m in moves {
        println!("{}", m);
    }
}
