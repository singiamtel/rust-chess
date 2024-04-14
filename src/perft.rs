#![allow(dead_code, unused)]
use crate::board::{gen_moves, Board};

pub fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = gen_moves(board);
    for m in moves.iter() {
        // make_move(board, m);
        // let nodes = perft(board, depth - 1);
        // unmake_move(board, m);
    }
    moves.len() as u64
}
