use super::{
    Expr,
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

impl<'a> Iterator for Parser<'a>
{
    type Item = Box<dyn Expr>;

    fn next(&mut self) -> Option<Box<dyn Expr>>
    {
        todo!();
    }
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
