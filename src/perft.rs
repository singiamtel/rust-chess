use crate::move_generation::Movegen;
use crate::Game;
use rayon::prelude::*;

pub fn perft(game: &mut Game, depth: u8, is_root: bool) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = game.board.gen_moves().unwrap();
    let mut all_nodes = 0;
    for m in &moves {
        game.make_move(*m);
        let nodes = if game.board.is_check(!game.board.turn) {
            0
        } else {
            perft(game, depth - 1, false)
        };
        game.unmake_move(*m);
        if is_root && nodes > 0 {
            println!("{m} {nodes}");
        }
        all_nodes += nodes;
    }
    all_nodes
}

pub fn perft_parallel(game: &Game, depth: u8, is_root: bool) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = game.board.gen_moves().unwrap();
    let all_nodes: u64 = moves
        .par_iter()
        .map_init(
            || game.clone(), // Initialize a clone of the game for each thread
            |game_clone, m| {
                game_clone.make_move(*m);
                let nodes = if game_clone.board.is_check(game_clone.board.turn) {
                    0
                } else {
                    perft(game_clone, depth - 1, false)
                };
                game_clone.unmake_move(*m);
                if is_root && nodes > 0 {
                    println!("{m} {nodes}");
                }
                nodes
            },
        )
        .sum();
    all_nodes
}

pub fn test_parallelism() {
    println!("Rayon is using {} threads", rayon::current_num_threads());
    (1..100000).into_par_iter().for_each(|x| {
        println!("X: {:?} Thread ID: {:?}", x, std::thread::current().id());
    });
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
