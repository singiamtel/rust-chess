#![allow(dead_code, unused)]
use crate::{game::gen_moves, game::Game};

pub fn perft(game: &Game, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = gen_moves(game);
    for m in &moves {
        // make_move(board, m);
        // let nodes = perft(board, depth - 1);
        // unmake_move(board, m);
    }
    moves.len() as u64
}
