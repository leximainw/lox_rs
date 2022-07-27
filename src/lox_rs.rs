mod lexer;
pub use self::lexer::Lexer as Lexer;

pub struct Token<'a>
{
    pub start: usize,
    pub text: &'a str,
    pub kind: TokenType
}

#[derive(Debug)]   // TODO: remove when not printing TokenType names
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

    Error
}