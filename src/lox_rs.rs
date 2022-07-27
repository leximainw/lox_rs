mod lexer;
mod npeekable;
pub use self::lexer::Lexer as Lexer;
pub use self::npeekable::NPeekable as NPeekable;
pub use self::npeekable::NPeekableExt as NPeekableExt;

#[derive(Debug)]   // TODO: remove Debug when not printing TokenType values
pub enum LoxValue
{
    Str(String),
    Num(f64),
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
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // double-character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifier, String, Number,

    // keywords
    And, Class, Else, False, Fn, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    // sentinels
    Error, EOF
}