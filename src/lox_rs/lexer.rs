use std::iter::Peekable;
use std::str::CharIndices;
use super::{
    LoxValue,
    Token,
    TokenType
};

pub struct Lexer<'a>
{
    source: &'a str,
    iter: Peekable<CharIndices<'a>>,
    index: usize,
    token_start: usize
}

impl Lexer<'_>
{
    pub fn new(source: &str) -> Lexer
    {
        Lexer{
            source: source,
            iter: source.char_indices().peekable(),
            index: 0,
            token_start: 0
        }
    }

    pub fn next(&mut self) -> Token
    {
        let char = self.advance_past_whitespace();
        match char
        {
            Some(c) =>
            {
                let (kind, value) = self.read_token(c);
                Token{
                    kind: kind,
                    start: self.token_start,
                    text: Self::split_range(self.source, self.token_start, self.index),
                    value: value
                }
            },
            None => Token{
                kind: TokenType::EOF,
                start: self.index,
                text: Self::split_range(self.source, self.index, self.index),
                value: LoxValue::Nil
            }
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

    fn advance_past_whitespace(&mut self) -> Option<char>
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
        char
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

    fn read_token(&mut self, char: char) -> (TokenType, LoxValue)
    {
        self.token_start = self.index;
        let kind = match char
        {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => if self.check('=') { TokenType::BangEqual } else { TokenType::Bang },
            '=' => if self.check('=') { TokenType::EqualEqual } else { TokenType::Equal },
            '<' => if self.check('=') { TokenType::LessEqual } else { TokenType::Less },
            '>' => if self.check('=') { TokenType::GreaterEqual } else { TokenType::Greater },
            '"' => TokenType::String,
            '0' ..= '9' => TokenType::Number,
            '/' =>
            {
                if self.check('/')
                {
                    while let Some(c) = self.advance()
                    {
                        if c == '\n'
                        {
                            break
                        }
                    }
                    match self.advance_past_whitespace()
                    {
                        Some(c) => return self.read_token(c),
                        None => TokenType::EOF
                    }
                }
                else
                {
                    TokenType::Slash
                }
            },
            _ =>
            {
                TokenType::Error
            }
        };
        match kind
        {
            TokenType::String =>
            {
                if let Some(str) = self.string()
                { (kind, LoxValue::Str(str)) }
                else { (TokenType::Error, LoxValue::Nil) }
            },
            TokenType::Number => (kind, LoxValue::Num(self.number(char))),
            _ => (kind, LoxValue::Nil)
        }
    }

    fn string(&mut self) -> Option<String>
    {
        let mut str = String::new();
        let mut escaped = false;
        while let Some(char) = self.advance()
        {
            match char
            {
                c if escaped == true =>
                {
                    str.push(c);
                    escaped = false;
                },
                '\\' => escaped = true,
                '"' => return Some(str),
                c => str.push(c)
            }
        }
        None
    }

    fn number(&mut self, char: char) -> f64
    {
        while let Some(char) = self.peek()
        {
            match char
            {
                '0' ..= '9' => self.advance(),
                _ => break
            };
        }
        let text = Self::split_range(self.source, self.token_start, self.index);
        if let Ok(num) = text.parse::<f64>() { num } else { f64::NAN }
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
