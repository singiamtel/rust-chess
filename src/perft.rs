use crate::Game;

fn perft_recursive(game: &mut Game, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = game.gen_moves();
    for m in &moves {
        game.make_move(*m);
        if !game.is_check(game.turn) {
            nodes += perft_recursive(game, depth - 1);
        }
        game.unmake_move(*m);
    }
    nodes
}

pub fn perft(game: &mut Game, depth: u8) -> u64 {
    let mut nodes = 0;
    let moves = game.gen_moves();
    for m in &moves {
        game.make_move(*m);
        let new_nodes = perft_recursive(game, depth - 1);
        println!("{m} {new_nodes}");
        nodes += new_nodes;
        game.unmake_move(*m);
    }
    nodes
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use crate::piece::Piece;
    use crate::r#move::Move;
    use std::mem;
    // https://www.chessprogramming.org/Perft_Results#Initial_Position
    pub fn sizes() {
        println!("Size of Piece: {}", mem::size_of::<Piece>());
        println!("Size of Move: {}", mem::size_of::<Move>());
    }

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
