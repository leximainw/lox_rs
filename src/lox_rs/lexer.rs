use std::{
    collections::HashMap,
    str::CharIndices
};
use super::{
    LoxValue,
    NPeekable,
    NPeekableExt,
    Token,
    TokenType
};

pub struct Lexer<'a>
{
    source: &'a str,
    iter: NPeekable<CharIndices<'a>>,
    index: usize,
    token_start: usize
}

impl Lexer<'_>
{
    pub fn new(source: &str) -> Lexer
    {
        Lexer{
            source,
            iter: source.char_indices().npeekable(),
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
                    kind,
                    start: self.token_start,
                    text: Self::split_range(self.source, self.token_start, self.index),
                    value
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
                self.index = index + 1;
                Some(char)
            },
            None => None
        }
    }

    fn advance_past_whitespace(&mut self) -> Option<char>
    {
        let mut char = self.advance();
        while let Some(c) = char
        {
            // TODO: if let chaining becomes stable, replace while predicate with:
            // let Some(c) = char && c.is_whitespace()
            if !c.is_whitespace()
            {
                break
            }
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

    fn peek_next(&mut self) -> Option<char>
    {
        let char = match self.iter.peek_next()
        {
            Some((_, c)) => Some(*c),
            None => None
        };
        self.iter.reset_cursor();
        char
    }

    fn read_token(&mut self, char: char) -> (TokenType, LoxValue)
    {
        self.token_start = self.index - 1;
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
            'A' ..= 'Z' | 'a' ..= 'z' | '_' => TokenType::Identifier,
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
            TokenType::Identifier => (self.identifier(char), LoxValue::Nil),
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
        self.integer();
        if self.peek() == Some('.')
            && (Some('0') ..= Some('9')).contains(&self.peek_next())
        {
            self.advance();
            self.integer();
        }
        let peek = self.peek();
        if peek == Some('E') || peek == Some('e')
            && (Some('0') ..= Some('9')).contains(&self.peek_next())
        {
            self.advance();
            self.integer();
        }
        let text = Self::split_range(self.source, self.token_start, self.index);
        if let Ok(num) = text.parse::<f64>() { num } else { f64::NAN }
    }

    fn integer(&mut self)
    {
        while let Some(char) = self.peek()
        {
            match char
            {
                '0' ..= '9' => self.advance(),
                _ => break
            };
        }
    }

    fn identifier(&mut self, char: char) -> TokenType
    {
        // TODO: break into module constant if issue 88674 is made stable
        // https://github.com/rust-lang/rust/issues/88674
        let lexer_keywords: HashMap<&str, TokenType> = HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fn", TokenType::Fn),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While)
        ]);

        while let Some(char) = self.peek()
        {
            match char
            {
                'A' ..= 'Z' | 'a' ..= 'z'
                    | '0' ..= '9' | '_' => {}
                _ => break
            }
            self.advance();
        }
        let text = Self::split_range(self.source, self.token_start, self.index);
        if lexer_keywords.contains_key(text)
        { lexer_keywords[text] }
        else { TokenType::Identifier }
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
