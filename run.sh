#!/usr/bin/env bash

cargo build -p tic-tac-5 && cargo run -p server --release
# RUST_LOG=debug cargo run -p server --release