use super::{
    errors::{
        Errors,
        Severity
    },
    expr::*,
    stmt::*,
    lexer::Lexer,
    NPeekable,
    NPeekableExt,
    TokenType
};

pub struct Parser<'a>
{
    source: &'a str,
    lexer: NPeekable<Lexer<'a>>,
    errors: Errors<'a>
}

impl<'a> Iterator for Parser<'a>
{
    type Item = Box<dyn Stmt>;

    fn next(&mut self) -> Option<Box<dyn Stmt>>
    {
        loop
        {
            if let Some(stmt) = self.declaration()
            {
                return Some(stmt);
            }
            else if self.errors.get_flag()
            {
                self.synchronize();
            }
            else
            {
                return None;
            }
        }
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
        self.lexer.unwrap().coalesce_errors(target);
    }

    fn synchronize(&mut self)
    {
        self.errors.set_flag(false);
        while let Some(token) = self.lexer.peek()
        {
            match token.kind
            {
                TokenType::Semicolon =>
                {
                    self.lexer.next();
                    return
                },
                TokenType::Class | TokenType::For
                | TokenType::Fn | TokenType::If
                | TokenType::Print | TokenType::Return
                | TokenType::Var | TokenType::While => return,
                _ => {}
            }
        }
    }

    fn declaration(&mut self) -> Option<Box<dyn Stmt>>
    {
        if let Some(token) = self.lexer.peek()
        {
            match token.kind
            {
                TokenType::Var => self.var_declaration(),
                _ => self.statement()
            }
        }
        else { None }
    }

    fn var_declaration(&mut self) -> Option<Box<dyn Stmt>>
    {
        if let Some(var) = self.lexer.next()
        {
            if let Some(name) = self.lexer.next_if(
                |token| token.kind == TokenType::Identifier)
            {
                if let Some(token) = self.lexer.next()
                {
                    match token.kind
                    {
                        TokenType::Semicolon => return Some(Box::new(VarStmt{
                            name: name.text.to_string(),
                            expr: None,
                            start: var.start,
                            len: token.start - var.start + 1
                        })),
                        TokenType::Equal =>
                        {
                            if let Some(expr) = self.expression()
                            {
                                if let Some(end) = self.lexer.next_if(
                                    |token| token.kind == TokenType::Semicolon)
                                { return Some(Box::new(VarStmt{
                                    name: name.text.to_string(),
                                    expr: Some(expr),
                                    start: var.start,
                                    len: end.start - var.start + 1
                                })); }
                                else
                                {
                                    self.errors.push("expected ; after expression",
                                        Severity::Error, token.start + token.text.len(), 0, true);
                                }
                            }
                            else
                            {
                                self.errors.push("expected expression after =",
                                    Severity::Error, token.start + token.text.len(), 0, true);
                            }
                        },
                        _ =>
                        {
                            self.errors.push("expected = or ; after name",
                                Severity::Error, name.start + name.text.len(), 0, true);
                        }
                    }
                }
            }
            else
            {
                self.errors.push("expected name after 'var'",
                    Severity::Error, var.start + var.text.len(), 0, true);
            }
            None
        }
        else { panic!(); }
    }

    fn statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        if let Some(token) = self.lexer.peek()
        {
            match token.kind
            {
                TokenType::LeftBrace =>
                {
                    let start = token.start;
                    let block = self.block_statement();
                    match block
                    {
                        Some((stmts, end)) => Some(Box::new(BlockStmt{
                            stmts,
                            start,
                            len: end - start
                        })),
                        None => None
                    }
                }
                TokenType::Print => self.print_statement(),
                _ => self.expr_statement()
            }
        }
        else { None }
    }

    fn block_statement(&mut self) -> Option<(Vec<Box<dyn Stmt>>, usize)>
    {
        let mut stmts = Vec::new();
        if let Some(brace) = self.lexer.next()
        {
            loop
            {
                if let Some(brace) = self.lexer.peek()
                {
                    match brace.kind
                    {
                        TokenType::RightBrace =>
                        {
                            let end = brace.start + 1;
                            self.lexer.next();
                            return Some((stmts, end));
                        },
                        _ =>
                        {
                            if let Some(stmt) = self.declaration()
                            { stmts.push(stmt); }
                            else { return None; }
                        }
                    }
                }
                else if stmts.len() != 0
                {
                    let last_stmt = &stmts[stmts.len() - 1];
                    self.errors.push("expected closing brace after block",
                        Severity::Error, last_stmt.start() + last_stmt.len(), 0, true);
                    return None;
                }
                else
                {
                    self.errors.push("expected closing brace after block",
                        Severity::Error, brace.start + 1, 0, true);
                    return None;
                }
            }
        }
        else { panic!(); }
    }

    fn expr_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        if let Some(expr) = self.expression()
        {
            if let Some(end) = self.lexer.next_if(
                |token| token.kind == TokenType::Semicolon)
            {
                let start = expr.start();
                return Some(Box::new(ExprStmt{
                    expr,
                    start: start,
                    len: end.start - start + 1
                }));
            }
            else
            {
                self.errors.push("expected semicolon after expression statement",
                    Severity::Error, expr.start() + expr.len(), 0, true);
            }
        }
        None
    }

    fn print_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        if let Some(print) = self.lexer.next()
        {
            if let Some(expr) = self.expression()
            {
                if let Some(end) = self.lexer.next_if(
                    |token| token.kind == TokenType::Semicolon)
                {
                    return Some(Box::new(PrintStmt{
                        expr,
                        start: print.start,
                        len: end.start - print.start + 1
                    }));
                }
                else
                {
                    self.errors.push("expected semicolon after print statement",
                        Severity::Error, expr.start() + expr.len(), 0, true);
                }
            }
            else
            {
                self.errors.push("expected expression statement after 'print'",
                    Severity::Error, print.start + print.text.len(), 0, true);
            }
            None
        }
        else { panic!(); }
    }

    fn expression(&mut self) -> Option<Box<dyn Expr>>
    {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(expr) = self.equality()
        {
            if let Some(equal) = self.lexer.next_if(
                |token| token.kind == TokenType::Equal)
            {
                if let Some(var) = expr.as_varget()
                {
                    if let Some(value) = self.assignment()
                    {
                        Some(Box::new(VarSet{
                            start: expr.start(),
                            len: value.start() - expr.start() + value.len(),
                            name: var.name.to_string(),
                            expr: value
                        }))
                    }
                    else
                    {
                        self.errors.push("expected value after assignment",
                            Severity::Error, equal.start + equal.text.len(), 0, true);
                        None
                    }
                }
                else
                {
                    self.errors.push("invalid assignment target",
                        Severity::Error, expr.start(), expr.len(), true);
                    None
                }
            }
            else { Some(expr) }
        }
        else { None }
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
                        start: left.start(),
                        len: right.start() - left.start() + right.len(),
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expected expression after operator",
                        Severity::Error, oper.start + oper.text.len(), 0, true);
                    return None;
                }
            }
            Some(left)
        }
        else
        {
            if let Some(token) = self.lexer.peek()
            {
                self.errors.push("expected expression",
                    Severity::Error, token.start, token.text.len(), true);
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
                        start: left.start(),
                        len: right.start() - left.start() + right.len(),
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expected expression after operator",
                        Severity::Error, oper.start + oper.text.len(), 0, true);
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
                        start: left.start(),
                        len: right.start() - left.start() + right.len(),
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expected expression after operator",
                        Severity::Error, oper.start + oper.text.len(), 0, true);
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
                        start: left.start(),
                        len: right.start() - left.start() + right.len(),
                        left,
                        oper: oper.kind,
                        right
                    })
                }
                else
                {
                    self.errors.push("expected expression after operator",
                        Severity::Error, oper.start + oper.text.len(), 0, true);
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
                    start: token.start,
                    len: expr.start() - token.start + expr.len(),
                    oper: token.kind,
                    expr
                }))
            }
            else
            {
                self.errors.push("expected expression after operator",
                    Severity::Error, token.start + token.text.len(), 0, true);
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
                    start: token.start,
                    len: token.text.len(),
                    value: token.value
                })),
                TokenType::Identifier => Some(Box::new(VarGet{
                    start: token.start,
                    len: token.text.len(),
                    name: token.text.to_string()
                })),
                TokenType::LeftParen =>
                {
                    if let Some(expr) = self.expression()
                    {
                        if let Some(rtoken) = self.lexer.next()
                        {
                            match rtoken.kind
                            {
                                TokenType::RightParen =>
                                    Some(Box::new(Grouping{
                                        start: token.start,
                                        len: rtoken.start - token.start + rtoken.text.len(),
                                        expr
                                    })),
                                _ =>
                                {
                                    self.errors.push("expected rparen after expression",
                                        Severity::Error, rtoken.start, 0, true);
                                    None
                                }
                            }
                        }
                        else
                        {
                            self.errors.push("expected rparen after expression",
                                Severity::Error, self.source.len(), 0, true);
                            None
                        }
                    }
                    else
                    {
                        self.errors.push("expected expression after lparen",
                            Severity::Error, token.start + token.text.len(), 0, true);
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
