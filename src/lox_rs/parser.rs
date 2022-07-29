use super::{
    errors::{
        Errors,
        Severity
    },
    expr::{
        Binary,
        Expr,
        Grouping,
        Literal,
        Unary
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

    pub fn coalesce_errors(&mut self, target: &mut Errors)
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
                }})
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
                    self.errors.push("expect expression after operator",
                        Severity::Error, oper.start, oper.text.len());
                    return None;
                }
            }
            Some(left)
        }
        else
        {
            if let Some(token) = self.lexer.peek()
            {
                self.errors.push("expect expression",
                    Severity::Error, token.start, token.text.len());
            }
            None
        }
    }

    fn comparison(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(mut left) = self.term()
        {
            while let Some(oper) = self.lexer.next_if(|token| {
                match token.kind
                {
                    TokenType::Less
                    | TokenType::LessEqual
                    | TokenType::Greater
                    | TokenType::GreaterEqual => true,
                    _ => false
                }})
            {
                if let Some(right) = self.term()
                {
                    left = Box::new(Binary{
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expect expression after operator",
                        Severity::Error, oper.start, oper.text.len());
                    return None;
                }
            }
            Some(left)
        }
        else { None }
    }

    fn term(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(mut left) = self.factor()
        {
            while let Some(oper) = self.lexer.next_if(|token| {
                match token.kind
                {
                    TokenType::Plus
                    | TokenType::Minus => true,
                    _ => false
                }})
            {
                if let Some(right) = self.factor()
                {
                    left = Box::new(Binary{
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expect expression after operator",
                        Severity::Error, oper.start, oper.text.len());
                    return None;
                }
            }
            Some(left)
        }
        else { None }
    }

    fn factor(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(mut left) = self.unary()
        {
            while let Some(oper) = self.lexer.next_if(|token| {
                match token.kind
                {
                    TokenType::Star
                    | TokenType::Slash
                    | TokenType::Percent => true,
                    _ => false
                }})
            {
                if let Some(right) = self.unary()
                {
                    left = Box::new(Binary{
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expect expression after operator",
                        Severity::Error, oper.start, oper.text.len());
                    return None;
                }
            }
            Some(left)
        }
        else { None }
    }

    fn unary(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(token) = self.lexer.next_if(|token| {
            match token.kind {
                TokenType::Bang
                | TokenType::Minus => true,
                _ => false
            }})
        {
            if let Some(expr) = self.unary()
            {
                Some(Box::new(Unary{
                    oper: token.kind,
                    expr
                }))
            }
            else
            {
                self.errors.push("expect expression after operator",
                    Severity::Error, token.start, token.text.len());
                None
            }
        }
        else { self.primary() }
    }

    fn primary(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(token) = self.lexer.next_if(|token| {
            match token.kind {
                TokenType::Literal
                | TokenType::Identifier
                | TokenType::LeftParen => true,
                _ => false
            }})
        {
            match token.kind {
                TokenType::Literal => Some(Box::new(Literal{
                    value: token.value
                })),
                TokenType::Identifier =>
                {
                    self.errors.push("identifiers not yet implemented",
                        Severity::Error, token.start, token.text.len());
                    None
                },
                TokenType::LeftParen =>
                {
                    if let Some(expr) = self.expression()
                    {
                        if let Some(rtoken) = self.lexer.next()
                        {
                            match rtoken.kind
                            {
                                TokenType::RightParen =>
                                    Some(Box::new(Grouping{expr})),
                                _ =>
                                {
                                    self.errors.push("expect rparen after expression",
                                        Severity::Error, rtoken.start, rtoken.text.len());
                                    None
                                }
                            }
                        }
                        else
                        {
                            self.errors.push("expect rparen after expression",
                                Severity::Error, self.source.len(), 0);
                            None
                        }
                    }
                    else
                    {
                        self.errors.push("expect expression after lparen",
                            Severity::Error, token.start, token.text.len());
                        None
                    }
                },
                _ => None
            }
        }
        else
        {
            None
        }
    }
}
