mod expr;
mod lexer;
mod npeekable;
mod parser;
pub use self::lexer::Lexer as Lexer;
pub use self::parser::Parser as Parser;
pub use self::expr::Expr as Expr;
use self::npeekable::NPeekable as NPeekable;
use self::npeekable::NPeekableExt as NPeekableExt;

#[derive(Debug)]   // TODO: remove Debug when not printing LoxValue values
pub enum LoxValue
{
    Bool(bool),
    Num(f64),
    Str(String),
    Nil
}

pub struct Token<'a>
{
    pub start: usize,
    pub text: &'a str,
    pub kind: TokenType,
    pub value: LoxValue
}

#[derive(Debug)]   // TODO: remove when not printing TokenType names
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq)]
pub enum TokenType
{
    // single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Percent, Plus, Semicolon,
    Slash, Star,

    // double-character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Literal, Identifier, Number, String,

    // keywords
    And, Class, Else, False, Fn, For, If, Nil,
    Or, Return, Super, This, True, Var, While,
    Print,   // TODO: remove once functions work

    // sentinels
    Error, EOF
}
