use super::{
    errors::{
        Errors,
        Severity
    },
    expr::*,
    stmt::*,
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
    errors: Errors<'a>
}

enum PatternElem
{
    Token(Token),
    Expr(Box<dyn Expr>),
    Stmt(Box<dyn Stmt>)
}

impl PatternElem
{
    pub fn as_token(self) -> Token
    {
        if let PatternElem::Token(kind) = self { kind }
        else { panic!("pattern element is not a token"); }
    }

    pub fn as_expr(self) -> Box<dyn Expr>
    {
        if let PatternElem::Expr(expr) = self { expr }
        else { panic!("pattern element is not an expression"); }
    }

    pub fn as_stmt(self) -> Box<dyn Stmt>
    {
        if let PatternElem::Stmt(stmt) = self { stmt }
        else { panic!("pattern element is not a statement"); }
    }
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

    fn try_match(&mut self, pattern: Vec<(Box<dyn FnOnce(&mut Parser) -> Option<PatternElem>>, (bool, &'static str))>)
        -> Result<Vec<PatternElem>, &'static str>
    {
        let mut curr_end = if let Some(token) = self.lexer.peek()
        { token.start + token.text.len() }
        else { self.source.len() };
        let mut elems: Vec<PatternElem> = Vec::new();
        for (f, err) in pattern
        {
            let (start, len) = if let Some(token) = self.lexer.peek()
            { (token.start, token.text.len()) }
            else
            { (self.source.len(), 0) };
            match f(self)
            {
                Some(elem) =>
                {
                    curr_end = pattern_end(&elem);
                    elems.push(elem);
                }
                None =>
                {
                    let (nz_len, err) = err;
                    if nz_len { self.errors.push(err, Severity::Error, start, len, true); }
                    else { self.errors.push(err, Severity::Error, curr_end, 0, true); }
                    return Err(err);
                }
            }
        }
        return Ok(elems);
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

    fn block(&mut self) -> Option<(Vec<Box<dyn Stmt>>, (usize, usize))>
    {
        let mut stmts = Vec::new();
        if let Some(brace) = self.lexer.next_if(
            |token| token.kind == TokenType::LeftBrace
        )
        {
            let start = brace.start;
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
                            return Some((stmts, (start, end - start)));
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
        else { None }
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
        match self.try_match(vec![
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::Var))),
                (true, "expected 'var'")),
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::Identifier))),
                (false, "expected name after 'var'"))
        ])
        {
            Err(_) => None,
            Ok(mut parts) => if let Some(token) = self.lexer.next_if(
                |token| token.kind == TokenType::Semicolon
                    || token.kind == TokenType::Equal
            )
            {
                if token.kind == TokenType::Semicolon
                {
                    Some(Box::new(VarStmt{
                        name: parts.remove(1).as_token().text,
                        expr: None,
                        start: pattern_start(&parts[0]),
                        len: token.start - pattern_start(&parts[0]) + 1
                    }))
                }
                else
                {
                    match self.try_match(vec![
                        (Box::new(|parser| pattern_expr(parser.expression())),
                            (false, "expected expression after =")),
                        (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::Semicolon))),
                            (false, "expected ; after expression"))
                    ])
                    {
                        Err(_) => None,
                        Ok(mut inner_parts) =>
                        {
                            Some(Box::new(VarStmt{
                                name: parts.remove(1).as_token().text,
                                expr: Some(inner_parts.remove(0).as_expr()),
                                start: pattern_start(&parts[0]),
                                len: pattern_end(&inner_parts[0]) - pattern_start(&parts[0])
                            }))
                        }
                    }
                }
            }
            else
            {
                self.errors.push("expected = or ; after name",
                    Severity::Error, pattern_end(&parts[1]), 0, true);
                None
            }
        }
    }

    fn statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        if let Some(token) = self.lexer.peek()
        {
            match token.kind
            {
                TokenType::LeftBrace => self.block_statement(),
                TokenType::For => self.for_statement(),
                TokenType::If => self.if_statement(),
                TokenType::Print => self.print_statement(),
                TokenType::While => self.while_statement(),
                _ =>
                {
                    let start = token.start;
                    let len = token.text.len();
                    if let Some(expr) = self.expr_statement()
                    {
                        Some(expr)
                    }
                    else
                    {
                        self.errors.push("expected statement",
                            Severity::Error, start, len, true);
                        self.lexer.next();
                        None
                    }
                }
            }
        }
        else { None }
    }

    fn block_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        if let Some(block) = self.block()
        {
            let (stmts, (start, len)) = block;
            Some(Box::new(BlockStmt{
                stmts,
                start,
                len
            }))
        }
        else { None }
    }

    fn expr_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        match self.try_match(vec![
            (Box::new(|parser| pattern_expr(parser.expression())),
                (true, "expected expression")),
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::Semicolon))),
                (false, "expected ; after expression statement"))
        ])
        {
            Err(_) => None,
            Ok(mut parts) =>
            {
                Some(Box::new(ExprStmt{
                    start: pattern_start(&parts[0]),
                    len: pattern_end(&parts[1]) - pattern_start(&parts[0]),
                    expr: parts.remove(0).as_expr()
                }))
            }
        }
    }

    fn for_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        match self.try_match(vec![
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::For))),
                (true, "expected 'for'")),
            (Box::new(|parser| pattern_stmt(if let Some(token) = parser.lexer.peek()
            {
                match token.kind
                {
                    TokenType::Semicolon => Some(Self::true_stmt()),
                    TokenType::Var => parser.var_declaration(),
                    _ => parser.expr_statement()
                }
            } else { None })),
                (false, "expected semicolon, 'var', or expression after 'for'")),
            (Box::new(|parser| pattern_stmt(if let Some(token) = parser.lexer.peek()
            {
                match token.kind
                {
                    TokenType::Semicolon => Some(Self::true_stmt()),
                    _ => parser.expr_statement()
                }
            } else { None })),
                (false, "expected semicolon or expression after initializer")),
            (Box::new(|parser| pattern_expr(if let Some(token) = parser.lexer.peek()
            {
                match token.kind
                {
                    TokenType::LeftBrace => Some(Self::true_expr()),
                    _ => parser.expression()
                }
            } else { None })),
                (false, "expected left brace or expression after condition")),
            (Box::new(|parser| pattern_stmt(parser.block_statement())),
                (false, "expected block after for statement"))
        ])
        {
            Err(_) => None,
            Ok(mut parts) =>
            {
                let start = pattern_start(&parts[0]);
                let len = pattern_end(&parts[4]) - start;
                Some(Box::new(BlockStmt{
                    stmts: vec![
                        parts.remove(1).as_stmt(),
                        Box::new(WhileStmt{
                            expr: parts.remove(1).as_stmt().to_exprstmt().unwrap().expr,
                            stmt: Box::new(BlockStmt{
                                stmts: vec![
                                    parts.remove(2).as_stmt(),
                                    Box::new(ExprStmt{
                                        expr: parts.remove(1).as_expr(),
                                        start,
                                        len
                                    }),
                                ],
                                start,
                                len
                            }),
                            start,
                            len
                        })
                    ],
                    start,
                    len
                }))
            }
        }
    }

    fn if_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        match self.try_match(vec![
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::If))),
                (true, "expected 'if'")),
            (Box::new(|parser| pattern_expr(parser.expression())),
                (false, "expected predicate after 'if'")),
            (Box::new(|parser| pattern_stmt(parser.block_statement())),
                (false, "expected block after if statement")),
        ])
        {
            Err(_) => None,
            Ok(mut parts) =>
            {
                match if let Some(_) = self.lexer.peek_if(
                    |token| token.kind == TokenType::Else
                )
                {
                    match self.try_match(vec![
                        (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::Else))),
                            (true, "expected 'else'")),   // this error should never happen
                        (Box::new(|parser| pattern_stmt(if let Some(_) = parser.lexer.peek_if(
                            |token| token.kind == TokenType::If
                        )
                        { parser.if_statement() }
                        else { parser.block_statement() })),
                            (false, "expected 'if' or block after else statement"))
                    ])
                    {
                        Err(_) => Err(()),
                        Ok(mut parts) => Ok((pattern_end(&parts[1]), Some(parts.remove(1).as_stmt())))
                    }
                }
                else { Ok((pattern_end(&parts[2]), None)) }
                {
                    Err(()) => None,
                    Ok((end, stmt_false)) => Some(Box::new(IfStmt{
                        start: pattern_start(&parts[0]),
                        len: end - pattern_start(&parts[0]),
                        expr: parts.remove(1).as_expr(),
                        stmt_true: parts.remove(1).as_stmt(),
                        stmt_false
                    }))
                }
            }
        }
    }

    fn print_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        match self.try_match(vec![
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::Print))),
                (true, "expected 'print'")),
            (Box::new(|parser| pattern_expr(parser.expression())),
                (false, "expected expression after 'print'")),
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::Semicolon))),
                (false, "expected semicolon after print statement"))
        ])
        {
            Err(_) => None,
            Ok(mut parts) =>
            {
                Some(Box::new(PrintStmt{
                    expr: parts.remove(1).as_expr(),
                    start: pattern_start(&parts[0]),
                    len: pattern_end(&parts[1]) - pattern_start(&parts[0])
                }))
            }
        }
    }

    fn while_statement(&mut self) -> Option<Box<dyn Stmt>>
    {
        match self.try_match(vec![
            (Box::new(|parser| pattern_token(parser.lexer.next_if(|token| token.kind == TokenType::While))),
                (true, "expected 'while'")),
            (Box::new(|parser| pattern_expr(parser.expression())),
                (false, "expected condition after 'while'")),
            (Box::new(|parser| pattern_stmt(parser.block_statement())),
                (false, "expected block after while statement"))
        ])
        {
            Err(_) => None,
            Ok(mut parts) =>
            {
                Some(Box::new(WhileStmt{
                    start: pattern_start(&parts[0]),
                    len: pattern_end(&parts[2]) - pattern_start(&parts[0]),
                    expr: parts.remove(1).as_expr(),
                    stmt: parts.remove(1).as_stmt()
                }))
            }
        }
    }

    fn expression(&mut self) -> Option<Box<dyn Expr>>
    {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(expr) = self.logic_or()
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

    fn logic_or(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(mut left) = self.logic_and()
        {
            while let Some(oper) = self.lexer.next_if(
                |token| { token.kind == TokenType::Or })
            {
                if let Some(right) = self.logic_and()
                {
                    left = Box::new(Logical{
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

    fn logic_and(&mut self) -> Option<Box<dyn Expr>>
    {
        if let Some(mut left) = self.equality()
        {
            while let Some(oper) = self.lexer.next_if(
                |token| { token.kind == TokenType::And })
            {
                if let Some(right) = self.equality()
                {
                    left = Box::new(Logical{
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
                    name: token.text
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

    fn true_stmt() -> Box<dyn Stmt>
    {
        // this statement should never be able to cause a runtime error
        // so it doesn't need accurate position information
        Box::new(ExprStmt{
            expr: Self::true_expr(),
            start: 0,
            len: 0
        })
    }

    fn true_expr() -> Box<dyn Expr>
    {
        // this expression should never be able to cause a runtime error
        // so it doesn't need accurate position information
        Box::new(Literal{
            value: LoxValue::Bool(true),
            start: 0,
            len: 0
        })
    }
}

fn pattern_start(elem: &PatternElem) -> usize
{
    match elem
    {
        PatternElem::Token(token) => token.start,
        PatternElem::Expr(expr) => expr.start(),
        PatternElem::Stmt(stmt) => stmt.start()
    }
}

fn pattern_len(elem: &PatternElem) -> usize
{
    match elem
    {
        PatternElem::Token(token) => token.text.len(),
        PatternElem::Expr(expr) => expr.len(),
        PatternElem::Stmt(stmt) => stmt.len()
    }
}

fn pattern_end(elem: &PatternElem) -> usize
{
    pattern_start(elem) + pattern_len(elem)
}

fn pattern_token(token: Option<Token>) -> Option<PatternElem>
{
    if let Some(token) = token
    { Some(PatternElem::Token(token)) }
    else { None }
}

fn pattern_expr(expr: Option<Box<dyn Expr>>) -> Option<PatternElem>
{
    if let Some(expr) = expr
    { Some(PatternElem::Expr(expr)) }
    else { None }
}

fn pattern_stmt(stmt: Option<Box<dyn Stmt>>) -> Option<PatternElem>
{
    if let Some(stmt) = stmt
    { Some(PatternElem::Stmt(stmt)) }
    else { None }
}
