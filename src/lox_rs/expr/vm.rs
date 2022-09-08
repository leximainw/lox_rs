use super::super::VM;
use super::super::Backtrace;
use super::*;

impl Visitor<Result<LoxValue, Backtrace>> for VM
{
    fn visit_binary(&mut self, expr: &Binary) -> Result<LoxValue, Backtrace>
    {
        match expr.left.run(self)
        {
            Ok(lval) => match expr.right.run(self)
            {
                Ok(rval) => match expr.oper
                {
                    TokenType::EqualEqual => Ok(LoxValue::Bool(lval == rval)),
                    TokenType::BangEqual => Ok(LoxValue::Bool(lval != rval)),
                    TokenType::Less | TokenType::LessEqual
                    | TokenType::Greater | TokenType::GreaterEqual =>
                    {
                        // TODO: merge if let chaining becomes stable
                        if let LoxValue::Str(ref lstr) = lval
                        {
                            if let LoxValue::Str(ref rstr) = rval
                            {
                                return match expr.oper
                                {
                                    TokenType::Less => Ok(LoxValue::Bool(lstr < rstr)),
                                    TokenType::LessEqual => Ok(LoxValue::Bool(lstr <= rstr)),
                                    TokenType::Greater => Ok(LoxValue::Bool(lstr > rstr)),
                                    TokenType::GreaterEqual => Ok(LoxValue::Bool(lstr >= rstr)),
                                    _ => panic!()
                                }
                            }
                        }
                        else if let LoxValue::Num(ref lnum) = lval
                        {
                            if let LoxValue::Num(ref rnum) = rval
                            {
                                return match expr.oper
                                {
                                    TokenType::Less => Ok(LoxValue::Bool(lnum < rnum)),
                                    TokenType::LessEqual => Ok(LoxValue::Bool(lnum <= rnum)),
                                    TokenType::Greater => Ok(LoxValue::Bool(lnum > rnum)),
                                    TokenType::GreaterEqual => Ok(LoxValue::Bool(lnum >= rnum)),
                                    _ => panic!()
                                };
                            }
                        }
                        match lval
                        {
                            LoxValue::Str(_)
                            | LoxValue::Num(_) =>
                            {
                                todo!()
                            },
                            _ =>
                            {
                                Err(Backtrace::starting_at(format!("expected number or string"),
                                    (expr.left.start(), expr.left.len())))
                            }
                        }
                    },
                    TokenType::Plus =>
                    {
                        const ERR: &str = "expected two numbers or two strings";
                        match lval
                        {
                            LoxValue::Num(lnum) => if let LoxValue::Num(rnum) = rval
                            { Ok(LoxValue::Num(lnum + rnum)) }
                            else { Err(Backtrace::starting_at(ERR.to_string(),
                                (expr.right.start(), expr.right.len()))) },
                            LoxValue::Str(lstr) => if let LoxValue::Str(rstr) = rval
                            { Ok(LoxValue::Str(format!("{}{}", lstr, rstr))) }
                            else { Err(Backtrace::starting_at(ERR.to_string(),
                                (expr.right.start(), expr.right.len()))) },
                            _ =>
                            {
                                Err(Backtrace::starting_at(ERR.to_string(),
                                    (expr.left.start(), expr.left.len())))
                            }
                        }
                    },
                    TokenType::Minus | TokenType::Star
                    | TokenType::Slash | TokenType::Percent =>
                    {
                        if let LoxValue::Num(lnum) = lval
                        {
                            if let LoxValue::Num(rnum) = rval
                            {
                                match expr.oper
                                {
                                    TokenType::Minus => Ok(LoxValue::Num(lnum - rnum)),
                                    TokenType::Star => Ok(LoxValue::Num(lnum * rnum)),
                                    TokenType::Slash => Ok(LoxValue::Num(lnum / rnum)),
                                    TokenType::Percent => Ok(LoxValue::Num(lnum - (lnum / rnum).floor() * rnum)),
                                    _ => panic!()
                                }
                            }
                            else
                            {
                                Err(Backtrace::starting_at(format!("expected two numbers"),
                                    (expr.right.start(), expr.right.len())))
                            }
                        }
                        else
                        {
                            Err(Backtrace::starting_at(format!("expected two numbers"),
                                (expr.left.start(), expr.left.len())))
                        }
                    },
                    _ => panic!()
                },
                Err(err) => Err(err)
            },
            Err(err) => Err(err)
        }
    }

    fn visit_call(&mut self, expr: &Call) -> Result<LoxValue, Backtrace>
    {
        let value = expr.callee.run(self);
        if let Ok(LoxValue::Fn(mut callee)) = value {
            match expr.args.iter().map(|arg| arg.run(self)).collect() {
                Ok(args) => callee.call(args),
                Err(err) => Err(err)
            }
        } else {
            Err(Backtrace::starting_at(format!("expected callable"), (expr.callee.start(), expr.callee.len())))
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<LoxValue, Backtrace>
    {
        expr.expr.run(self)
    }

    fn visit_literal(&mut self, expr: &Literal) -> Result<LoxValue, Backtrace>
    {
        match &expr.value
        {
            LoxValue::Bool(value) => Ok(LoxValue::Bool(*value)),
            LoxValue::Num(value) => Ok(LoxValue::Num(*value)),
            LoxValue::Str(value) => Ok(LoxValue::Str(value.to_string())),
            LoxValue::Fn(value) => Ok(LoxValue::Fn(*value)),
            LoxValue::Nil => Ok(LoxValue::Nil)
        }
    }

    fn visit_logical(&mut self, expr: &Logical) -> Result<LoxValue, Backtrace>
    {
        match expr.left.run(self)
        {
            Ok(lval) =>
            {
                let lbool = LoxValue::is_truthy(&lval);
                match expr.oper
                {
                    TokenType::And if !lbool => return Ok(lval),
                    TokenType::Or if lbool => return Ok(lval),
                    _ => {}
                }
            },
            Err(err) => return Err(err)
        }
        match expr.right.run(self)
        {
            Ok(rval) => Ok(rval),
            Err(err) => Err(err)
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<LoxValue, Backtrace>
    {
        match expr.oper
        {
            TokenType::Bang => match expr.expr.run(self)
            {
                Ok(value) => Ok(LoxValue::Bool(!LoxValue::is_truthy(&value))),
                Err(err) => Err(err)
            },
            TokenType::Minus =>
            {
                match expr.expr.run(self)
                {
                    Ok(value) => if let LoxValue::Num(num) = value
                    {
                        Ok(LoxValue::Num(-num))
                    }
                    else { Err(Backtrace::starting_at(format!("expected number"),
                        (expr.expr.start(), expr.expr.len()))) },
                    Err(err) => Err(err)
                }
            }
            _ => panic!()
        }
    }

    fn visit_varget(&mut self, expr: &VarGet) -> Result<LoxValue, Backtrace>
    {
        if let Some(value) = self.curr_scope.get(&expr.name)
        {
            match value
            {
                LoxValue::Bool(value) => Ok(LoxValue::Bool(*value)),
                LoxValue::Num(value) => Ok(LoxValue::Num(*value)),
                LoxValue::Str(value) => Ok(LoxValue::Str(value.to_string())),
                LoxValue::Fn(value) => Ok(LoxValue::Fn(*value)),
                LoxValue::Nil => Ok(LoxValue::Nil)
            }
        }
        else { Err(Backtrace::starting_at(format!("undefined variable"),
            (expr.start(), expr.len()))) }
    }

    fn visit_varset(&mut self, expr: &VarSet) -> Result<LoxValue, Backtrace>
    {
        match expr.expr.run(self)
        {
            Ok(value) =>
            {
                if self.curr_scope.set(expr.name.to_string(), match value
                {
                    LoxValue::Bool(value) => LoxValue::Bool(value),
                    LoxValue::Num(value) => LoxValue::Num(value),
                    LoxValue::Str(ref value) => LoxValue::Str(value.to_string()),
                    LoxValue::Fn(value) => LoxValue::Fn(value),
                    LoxValue::Nil => LoxValue::Nil
                })
                {
                    Ok(value)
                }
                else { Err(Backtrace::starting_at(format!("undefined variable"),
                    (expr.start(), expr.len()))) }
            }
            Err(err) => Err(err)
        }
    }
}
