use perft::perft;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new(Game::STARTING_FEN);
    // let mut game = Game::new("8/8/8/8/8/7N/8/8 w HAha - 0 1");
    let perft_depth = env::args()
        .nth(1)
        .unwrap_or("2".to_string())
        .parse::<u8>()
        .unwrap();

    let n_moves = perft(&mut game, perft_depth)?;
    println!("Total moves: {n_moves}");
    // sizes();
    Ok(())
}

#[cfg(test)]
mod tests {
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
        let mut game = Game::new(Game::STARTING_FEN);
        // TODO: Test all the way down!
        for depth in 1..=2 {
            let n_moves = perft(&mut game, depth).unwrap();
            assert_eq!(
                n_moves,
                PERFT_RESULTS[depth as usize - 1],
                "Perft failed at depth {}",
                depth
            );
        }
    }
}
