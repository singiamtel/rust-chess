use perft::perft;

use crate::{game::Game, test::sizes};

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
    // println!("{}", game.board.white);
    // let coords = [4, 0];
    // let moves: Vec<Move> = gen_moves_from_piece(&board, Bitboard::FROM_SQUARE(coords));
    // let piece = match board.get_piece(Bitboard::FROM_SQUARE(coords)) {
    //     Some(p) => p,
    //     None => panic!("No piece found"),
    // };
    // let moves = gen_moves(&game);
    // count how many moves are generated per piece
    // let mut move_count = 0;
    // let mut pawn_moves = 0;
    // let mut knight_moves = 0;
    // let mut bishop_moves = 0;
    // let mut rook_moves = 0;
    // let mut queen_moves = 0;
    // let mut king_moves = 0;
    // for m in &moves {
    //     move_count += 1;
    //     match game.board.get_piece(m.from).unwrap().kind {
    //         PieceKind::Pawn => pawn_moves += 1,
    //         PieceKind::Knight => knight_moves += 1,
    //         PieceKind::Bishop => bishop_moves += 1,
    //         PieceKind::Rook => rook_moves += 1,
    //         PieceKind::Queen => queen_moves += 1,
    //         PieceKind::King => king_moves += 1,
    //     };
    // }
    // println!("Total legal moves in turn 1: {move_count}");
    // println!("Pawn: {pawn_moves}");
    // println!("Knight: {knight_moves}");
    // println!("Bishop: {bishop_moves}");
    // println!("Rook: {rook_moves}");
    // println!("Queen: {queen_moves}");
    // println!("King: {king_moves}");
    let n_moves = perft(&mut game, 2)?;
    println!("Total moves: {n_moves}");
    sizes();
    Ok(())
}
