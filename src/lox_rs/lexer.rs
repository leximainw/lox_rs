pub use self::lexer::Lexer;

use super::Token;
use super::TokenType;

mod lexer
{
    use std::iter::Peekable;
    use std::str::CharIndices;
    use super::Token;
    use super::TokenType;

    pub struct Lexer<'a>
    {
        source: &'a str,
        iter: Peekable<CharIndices<'a>>,
        index: usize
    }

    impl Lexer<'_>
    {
        pub fn new(source: &str) -> Lexer
        {
            Lexer{
                source: source,
                iter: source.char_indices().peekable(),
                index: 0
            }
        }

        pub fn next(&mut self) -> Option<Token>
        {
            let char = self.advance_past_whitespace();
            match char
            {
                Some((i, c)) =>
                {
                    Some(Token{
                        start: i,
                        kind: self.read_token(c),
                        text: Self::split_range(self.source, i, self.index)
                    })
                },
                None => None
            }
        }

        fn advance(&mut self) -> Option<char>
        {
            match self.iter.next()
            {
                Some((index, char)) =>
                {
                    self.index = index;
                    Some(char)
                },
                None => None
            }
        }

        fn advance_past_whitespace(&mut self) -> Option<(usize, char)>
        {
            let mut index = self.index;
            let mut char = self.advance();
            while let Some(c) = char
            {
                // TODO: if let chaining becomes stable, replace while predicate with:
                // let Some(c) = char && c.is_whitespace()
                if !c.is_whitespace()
                {
                    break
                }
                index = self.index;
                char = self.advance();
            }
            match char
            {
                Some(c) => Some((index, c)),
                None => None
            }
        }

        fn check(&mut self, char: char) -> bool
        {
            match self.peek()
            {
                Some(c) if c == char =>
                {
                    self.advance();
                    true
                },
                _ => false
            }
        }

        fn peek(&mut self) -> Option<char>
        {
            match self.iter.peek()
            {
                Some((_, c)) => Some(*c),
                None => None
            }
        }

        fn read_token(&mut self, char: char) -> TokenType
        {
            match char
            {
                '*' => TokenType::Star,
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '/' => TokenType::Slash,
                _ => TokenType::Error
            }
        }

        /// split_range: split a string at the provided start and end index.
        /// Panics if end comes before start, end is out of bounds, or start or end are not codepoint-aligned.
        fn split_range(text: &str, start: usize, end: usize) -> &str
        {
            debug_assert!(start <= end);
            let (pre, _) = text.split_at(end);
            let (_, post) = pre.split_at(start);
            post
        }
    }
}