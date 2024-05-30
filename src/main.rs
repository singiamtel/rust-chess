#![allow(clippy::missing_errors_doc)]

use crate::game::Game;
use std::env;

mod bitboard;
mod board;
mod eval;
mod game;
mod r#move;
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
        if !game.is_check(game.turn) {
            nodes += perft(game, depth - 1);
        }
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
    use self::bitboard::generate_pawn_lookup;

    use super::*;

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
    #[test]
    fn pawn_attacks_lookup() {
        // aaa
        let mine = generate_pawn_lookup();

        let correct: [[u64; 64]; 2] = [
            [
                0x200,
                0x500,
                0xa00,
                0x1400,
                0x2800,
                0x5000,
                0xa000,
                0x4000,
                0x20000,
                0x50000,
                0xa0000,
                0x140000,
                0x280000,
                0x500000,
                0xa00000,
                0x400000,
                0x2000000,
                0x5000000,
                0xa000000,
                0x14000000,
                0x28000000,
                0x50000000,
                0xa0000000,
                0x40000000,
                0x200000000,
                0x500000000,
                0xa00000000,
                0x1400000000,
                0x2800000000,
                0x5000000000,
                0xa000000000,
                0x4000000000,
                0x20000000000,
                0x50000000000,
                0xa0000000000,
                0x140000000000,
                0x280000000000,
                0x500000000000,
                0xa00000000000,
                0x400000000000,
                0x2000000000000,
                0x5000000000000,
                0xa000000000000,
                0x14000000000000,
                0x28000000000000,
                0x50000000000000,
                0xa0000000000000,
                0x40000000000000,
                0x200000000000000,
                0x500000000000000,
                0xa00000000000000,
                0x1400000000000000,
                0x2800000000000000,
                0x5000000000000000,
                0xa000000000000000,
                0x4000000000000000,
                0x0,
                0x0,
                0x0,
                0x0,
                0x0,
                0x0,
                0x0,
                0x0,
            ],
            [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0x2,
                0x5,
                0xa,
                0x14,
                0x28,
                0x50,
                0xa0,
                0x40,
                0x200,
                0x500,
                0xa00,
                0x1400,
                0x2800,
                0x5000,
                0xa000,
                0x4000,
                0x20000,
                0x50000,
                0xa0000,
                0x140000,
                0x280000,
                0x500000,
                0xa00000,
                0x400000,
                0x2000000,
                0x5000000,
                0xa000000,
                0x14000000,
                0x28000000,
                0x50000000,
                0xa0000000,
                0x40000000,
                0x200000000,
                0x500000000,
                0xa00000000,
                0x1400000000,
                0x2800000000,
                0x5000000000,
                0xa000000000,
                0x4000000000,
                0x20000000000,
                0x50000000000,
                0xa0000000000,
                0x140000000000,
                0x280000000000,
                0x500000000000,
                0xa00000000000,
                0x400000000000,
                0x2000000000000,
                0x5000000000000,
                0xa000000000000,
                0x14000000000000,
                0x28000000000000,
                0x50000000000000,
                0xa0000000000000,
                0x40000000000000,
            ],
        ];
        let correct_mapped = correct
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<u64>>>();
        let mine_vec = mine
            .iter()
            .map(|x| x.iter().map(|x| x.0).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>();
        assert_eq!(mine_vec, correct_mapped);
    }
}
