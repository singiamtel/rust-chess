use crate::board::Move;
use crate::piece::Piece;
use std::mem;

pub fn sizes() {
    println!("Size of Piece: {}", mem::size_of::<Piece>());
    println!("Size of Move: {}", mem::size_of::<Move>());
}
