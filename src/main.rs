#![allow(clippy::missing_errors_doc)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;

use rust_chess::perft::{perft, perft_parallel};
use rust_chess::Game;

fn main() -> Result<(), Box<dyn Error>> {
    let perft_depth = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("4"))
        .parse::<u8>()?;

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
    println!("\n{n_moves}");
    Ok(())
}
