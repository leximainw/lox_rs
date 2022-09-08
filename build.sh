#!/bin/zsh

# format source code
rustfmt ./src/bin/eof_newline.rs
rustfmt ./src/bin/generate_ast.rs
rustfmt ./src/main.rs

# rebuild autogenerators
cargo build

# make EOF newlines consistent
./target/debug/eof_newline ./src

# run autogenerators on targets
./target/debug/generate_ast ./src/lox_rs/expr.rs ./src/lox_rs/stmt.rs

# rebuild lox_rs
cargo build
