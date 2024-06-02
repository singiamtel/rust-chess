#![allow(clippy::missing_errors_doc)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;

use rust_chess::perft::{perft, perft_parallel, test_parallelism};
use rust_chess::Game;

fn main() -> Result<(), Box<dyn Error>> {
    const DEFAULT_DEPTH: u8 = 4;
    color_eyre::install()?;

    let perft_depth = if let Some(depth) = env::args().nth(1) {
        let perft_depth = depth.parse::<u8>()?;
        // if we received depth 0, use default
        if perft_depth == 0 {
            DEFAULT_DEPTH
        } else {
            perft_depth
        }
    } else {
        DEFAULT_DEPTH
    };

    let fen = env::args()
        .nth(2)
        .unwrap_or_else(|| Game::STARTING_FEN.to_string());
    let moves: String = env::args().nth(3).unwrap_or_default();
    let mut game = Game::new(&fen)?;

    if !moves.is_empty() {
        for mv in moves.split_whitespace() {
            let mv = game.parse_move(mv)?;
            game.make_move(mv);
        }
    }

    let n_moves = perft_parallel(&game, perft_depth, true);
    // let n_moves = perft(&mut game, perft_depth, true);
    // test_parallelism();
    println!("\n{n_moves}");
    Ok(())
}
