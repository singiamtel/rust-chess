use crate::{
    game::{make_move, parse_move, Game},
    perft::perft_divided,
};
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
    // let mut game = Game::new("8/8/8/8/8/7N/8/8 w HAha - 0 1");
    let perft_depth = env::args()
        .nth(1)
        .unwrap_or("2".to_string())
        .parse::<u8>()
        .unwrap();

    let fen = env::args().nth(2).unwrap_or(Game::STARTING_FEN.to_string());
    let moves = env::args().nth(3).unwrap_or("".to_string());

    let mut game = Game::new(&fen);

    if !moves.is_empty() {
        for mv in moves.split_whitespace() {
            let mv = parse_move(&mut game, mv).unwrap();
            make_move(&mut game, mv);
        }
    }

    let n_moves = perft_divided(&mut game, perft_depth)?;
    // let n_moves = perft_divided(&mut game, perft_depth)?;
    println!();
    println!("{n_moves}");
    // sizes();
    Ok(())
}

#[cfg(test)]
mod tests {
    // use crate::perft::perft;
    // use crate::Game;
    use crate::{game::Game, perft::perft};

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
        for depth in 1..=3 {
            let n_moves = perft(&mut game, depth).unwrap();
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
