use crate::Game;
use rayon::prelude::*;

pub fn perft(game: &mut Game, depth: u8, is_root: bool) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = game.gen_moves();
    let mut all_nodes = 0;
    for m in &moves {
        game.make_move(*m);
        let nodes = if !game.is_check() {
            // println!("{m} is not check");
            perft(game, depth - 1, false)
        } else {
            // println!("{m} is check");
            0
        };
        game.unmake_move(*m);
        if is_root {
            println!("{m} {nodes}");
        }
        all_nodes += nodes;
    }
    all_nodes
}

pub fn perft_parallel(game: &mut Game, depth: u8, is_root: bool) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = game.gen_moves();
    let all_nodes: u64 = moves
        .par_iter()
        .map_init(
            || game.clone(), // Initialize a clone of the game for each thread
            |game_clone, m| {
                game_clone.make_move(*m);
                let nodes = if !game_clone.is_check() {
                    perft(game_clone, depth - 1, false)
                } else {
                    0
                };
                game_clone.unmake_move(*m);
                if is_root {
                    println!("{m} {nodes}");
                }
                nodes
            },
        )
        .sum();
    all_nodes
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use crate::piece::Piece;
    use crate::r#move::Move;
    use std::mem;
    pub fn sizes() {
        println!("Size of Piece: {}", mem::size_of::<Piece>());
        println!("Size of Move: {}", mem::size_of::<Move>());
    }

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
        for depth in 1..=4 {
            let n_moves = perft(&mut game, depth, true);
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
