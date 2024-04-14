use crate::bitboard::Bitboard;

pub fn display_bitboard(bitboard: Bitboard) -> Vec<String> {
    let mut board = [['.'; 8]; 8];
    for i in 0..64 {
        if bitboard.0 & (1 << i) != 0 {
            let row = i / 8;
            let col = i % 8;
            board[row][col] = 'X';
        }
    }
    board
        .iter()
        .rev()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
}
