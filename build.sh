#!/bin/zsh

# rebuild autogenerators
cargo build

# run autogenerators on targets
./target/debug/generate_ast ./src/lox_rs/expr.rs

# rebuild lox_rs
cargo build
