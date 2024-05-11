#!/usr/bin/env bash

DEPTH=$1

if [ -z "$DEPTH" ]; then
  echo "Usage: $0 <depth>"
  exit 1
fi

difft <(stockfish go perft $DEPTH | head -n -3 | tail -n +2 | sort) <(cargo run $DEPTH 2>/dev/null | head -n -1 | sort)
