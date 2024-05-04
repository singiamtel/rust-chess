#![allow(dead_code, unused)]

use std::fmt::Error;

use crate::game::{gen_moves, make_move, unmake_move, Game};

pub fn perft(game: &mut Game, depth: u8) -> Result<u64, Error> {
    if depth == 0 {
        return Ok(1);
    }
    let mut nodes = 0;
    let moves = gen_moves(game);
    for (i, m) in moves.iter().enumerate() {
        // println!("{} {}", game.board.get_piece(m.from).ok_or(Error)?, m);
        // println!("Making the move {i}: {m}");
        make_move(game, *m);
        // println!("{}", game.board);
        // println!("{}", game.board.white);
        // println!("{}", game.board.black);
        nodes += perft(game, depth - 1)?;
        // println!("Unmaking move {i}: {m}");
        unmake_move(game, *m);
    }
    Ok(nodes)
}
