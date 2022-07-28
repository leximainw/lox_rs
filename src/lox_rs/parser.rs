use super::{
    Lexer,
    LoxValue,
    NPeekable,
    NPeekableExt,
    Token,
    TokenType
};

pub struct Parser<'a>
{
    source: &'a str,
    lexer: NPeekable<Lexer<'a>>
}

impl Parser<'_>
{
    pub fn new(source: &str) -> Parser
    {
        Parser{
            source,
            lexer: Lexer::new(source).npeekable()
        }
    }
}
