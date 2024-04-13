use crate::board::Bitboard;

// def display_chessboard(u64):
//     # Create a mapping of bits to chessboard squares
//     board = [["." for _ in range(8)] for _ in range(8)]

//     # Populate the chessboard based on the U64 input
//     for i in range(64):
//         if u64 & (1 << i):
//             # Calculate row and column based on Little-Endian Rank-File (LERF) mapping
//             row = i // 8
//             col = i % 8
//             board[row][col] = "X"  # Mark the square as occupied

//     # Print the chessboard
//     for row in reversed(board):  # Print from the 8th rank to the 1st
//         print(" ".join(row))

//     print("")

pub fn display_bitboard(bitboard: &Bitboard) -> Vec<String> {
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
