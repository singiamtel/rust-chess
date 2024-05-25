#![allow(clippy::missing_errors_doc)]

use crate::game::Game;
use std::env;

mod bitboard;
mod board;
mod eval;
mod game;
mod r#move;
mod perft;
mod piece;
mod printer;
mod test;

use std::error::Error;

pub fn perft(game: &mut Game, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = game.gen_moves();
    for m in &moves {
        game.make_move(*m);
        nodes += perft(game, depth - 1);
        game.unmake_move(*m);
    }
    nodes
}

pub fn perft_divided(game: &mut Game, depth: u8) -> u64 {
    let mut nodes = 0;
    let moves = game.gen_moves();
    for m in &moves {
        game.make_move(*m);
        let new_nodes = perft(game, depth - 1);
        println!("{m} {new_nodes}");
        nodes += new_nodes;
        game.unmake_move(*m);
    }
    nodes
}

fn main() -> Result<(), Box<dyn Error>> {
    let perft_depth = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("4"))
        .parse::<u8>()?;

    let fen = env::args()
        .nth(2)
        .unwrap_or_else(|| Game::STARTING_FEN.to_string());
    let moves = env::args().nth(3).unwrap_or_default();

    let mut game = Game::new(&fen)?;

    if !moves.is_empty() {
        for mv in moves.split_whitespace() {
            let mv = game.parse_move(mv)?;
            game.make_move(mv);
        }
    }

    let n_moves = perft_divided(&mut game, perft_depth);
    println!("\n{n_moves}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;

    // https://www.chessprogramming.org/Perft_Results#Initial_Position
    const PERFT_RESULTS: [u64; 9] = [
        20,
        400,
        8902,
        197281,
        4865609,
        119060324,
        3195901860,
        84998978956,
        2439530234167,
    ];

    #[test]
    fn perft_test() {
        let mut game = Game::new(Game::STARTING_FEN).unwrap();
        // TODO: Test all the way down!
        for depth in 1..=3 {
            let n_moves = perft(&mut game, depth);
            assert_eq!(
                n_moves,
                PERFT_RESULTS[depth as usize - 1],
                "Perft failed at depth {} (expected: {} but got: {})",
                depth,
                PERFT_RESULTS[depth as usize - 1],
                n_moves
            );
        }
    }
}
