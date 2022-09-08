pub mod backtrace;
pub mod closure;
pub mod errors;
mod expr;
mod lexer;
mod npeekable;
mod parser;
mod stmt;
pub mod vm;
pub use self::backtrace::Backtrace;
pub use self::closure::LoxClosure;
pub use self::errors::Errors;
pub use self::expr::Expr;
pub use self::lexer::Lexer;
use self::npeekable::NPeekable;
use self::npeekable::NPeekableExt;
pub use self::parser::Parser;
pub use self::vm::VM;

#[derive(Debug, PartialEq)]
pub enum LoxValue {
    Bool(bool),
    Num(f64),
    Str(String),
    Fn(LoxClosure),
    Nil,
}

pub struct Token {
    pub start: usize,
    pub text: String,
    pub kind: TokenType,
    pub value: LoxValue,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Percent,
    Plus,
    Semicolon,
    Slash,
    Star,

    // double-character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Literal,
    Identifier,
    Number,
    String,

    // keywords
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Print, // TODO: remove once functions work

    // sentinels
    Error,
    EOF,
}

impl LoxValue {
    pub fn is_truthy(value: &LoxValue) -> bool {
        match value {
            LoxValue::Bool(value) => *value,
            LoxValue::Num(_) => true,
            LoxValue::Str(_) => true,
            LoxValue::Fn(_) => true,
            LoxValue::Nil => false,
        }
    }
}

impl std::fmt::Display for LoxValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            LoxValue::Bool(value) => formatter.write_str(&value.to_string()),
            LoxValue::Num(value) => formatter.write_str(&value.to_string()),
            LoxValue::Str(value) => formatter.write_str(&value),
            LoxValue::Fn(value) => formatter.write_str("<function>"),
            LoxValue::Nil => formatter.write_str("nil"),
        }
    }
}
