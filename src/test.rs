#![allow(dead_code, unused)]
use crate::piece::Piece;
use crate::r#move::Move;
use std::mem;

pub fn sizes() {
    println!("Size of Piece: {}", mem::size_of::<Piece>());
    println!("Size of Move: {}", mem::size_of::<Move>());
}
