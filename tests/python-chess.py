#!/usr/bin/env python3

import chess
import sys

FEN = chess.STARTING_FEN

board = chess.Board(FEN)

# moves = [x.uci() for x in board.legal_moves]

moves = []
def perft(board, depth):
    if depth == 0:
        return 1

    nodes = 0
    moves.extend([x.uci() for x in board.legal_moves])
    for move in board.legal_moves:
        board.push(move)
        nodes += perft(board, depth - 1)
        board.pop()
    return nodes

n_moves = perft(board, int(sys.argv[1]))
for move in moves:
    print(move)

# print(n_moves)
