#!/usr/bin/env bash

# For https://github.com/agausmann/perftree


RUST_BACKTRACE=1 cargo run --release -- "$1" "$2" "$3"
