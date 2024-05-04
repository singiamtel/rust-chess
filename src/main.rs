use perft::perft;

use crate::game::Game;

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
    let n_moves = perft(&mut game, 3)?;
    println!("Total moves: {n_moves}");
    // sizes();
    Ok(())
}
