mod lexer;
pub use self::lexer::Lexer as Lexer;

pub struct Token<'a>
{
    pub start: usize,
    pub text: &'a str,
    pub kind: TokenType
}

pub enum TokenType
{
    None
}