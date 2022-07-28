use super::{
    errors::{
        Errors,
        Severity
    },
    expr::{
        Binary,
        Expr,
    },
    lexer::Lexer,
    LoxValue,
    NPeekable,
    NPeekableExt,
    Token,
    TokenType
};

pub struct Parser<'a>
{
    source: &'a str,
    lexer: NPeekable<Lexer<'a>>,
    errors: Errors<'a>,
}

impl<'a> Iterator for Parser<'a>
{
    type Item = Box<dyn Expr>;

    fn next(&mut self) -> Option<Box<dyn Expr>>
    {
        self.expression()
    }
}

impl Parser<'_>
{
    pub fn new<'a>(source: &'a str) -> Parser<'a>
    {
        Parser{
            source,
            lexer: Lexer::new(source).npeekable(),
            errors: Errors::new(source)
        }
    }

    fn coalesce_errors(&mut self, target: &mut Errors)
    {
        self.errors.coalesce(target);
        self.lexer.unwrap_mut().coalesce_errors(target);
    }

    fn expression(&mut self) -> Option<Box<dyn Expr>>
    {
        self.equality()
    }

    fn equality(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(mut left) = self.comparison()
        {
            while let Some(oper) = self.lexer.next_if(|token| {
                match token.kind
                {
                    TokenType::EqualEqual
                    | TokenType::BangEqual => true,
                    _ => false
                }
            })
            {
                if let Some(right) = self.comparison()
                {
                    left = Box::new(Binary{
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expect comparison after operator",
                        Severity::Error, oper.start, oper.text.len());
                    return None;
                }
            }
            Some(left)
        }
        else { None }
    }

    fn comparison(&mut self) -> Option<Box<dyn Expr>>
    {
        todo!()
    }
}
