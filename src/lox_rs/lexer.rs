pub use self::lexer::Lexer;

use super::Token;
use super::TokenType;

mod lexer
{
    use std::str::Chars;
    use super::Token;
    use super::TokenType;

    pub struct Lexer<'a>
    {
        source: &'a str,
        iter: Chars<'a>
    }

    impl Lexer<'_>
    {
        pub fn new(source: &str) -> Lexer
        {
            Lexer
            {
                source,
                iter: source.chars()
            }
        }

        pub fn next(&mut self) -> Option<Token>
        {
            None
        }
    }
}