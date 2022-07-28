REM rebuild autogenerators
cargo build

REM run autogenerators on targets
./target/debug/generate_ast ./src/lox_rs/expr.rs

REM rebuild lox_rs
cargo build
