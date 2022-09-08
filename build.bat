REM format source code
rustfmt ./src/bin/eof_newline.rs
rustfmt ./src/bin/generate_ast.rs
rustfmt ./src/main.rs

REM rebuild autogenerators
cargo build

REM make EOF newlines consistent
./target/debug/eof_newline ./src

REM run autogenerators on targets
./target/debug/generate_ast ./src/lox_rs/expr.rs ./src/lox_rs/stmt.rs

REM rebuild lox_rs
cargo build
