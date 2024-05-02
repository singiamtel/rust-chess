#![allow(dead_code, unused)]
use crate::game::{gen_moves, make_move, unmake_move, Game};

pub fn perft(game: &mut Game, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = gen_moves(game);
    moves
        .iter()
        .for_each(|m| println!("{} {:?}", game.board.get_piece(m.from).unwrap(), m));
    for m in &moves {
        make_move(game, *m);
        let nodes = perft(game, depth - 1);
        unmake_move(game);
    }
    moves.len() as u64
}
