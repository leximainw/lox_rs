use super::super::VM;
use super::*;

impl Visitor<Result<LoxValue, (&'static str, (usize, usize))>> for VM
{
    fn visit_binary(&mut self, expr: &Binary) -> Result<LoxValue, (&'static str, (usize, usize))>
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
                                Err(("expected number or string",
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
                            else { Err((ERR, (expr.right.start(), expr.right.len()))) },
                            LoxValue::Str(lstr) => if let LoxValue::Str(rstr) = rval
                            { Ok(LoxValue::Str(format!("{}{}", lstr, rstr))) }
                            else { Err((ERR, (expr.right.start(), expr.right.len()))) },
                            _ => Err((ERR, (expr.left.start(), expr.left.len())))
                        }
                    },
                    TokenType::Minus | TokenType::Star
                    | TokenType::Slash | TokenType::Percent =>
                    {
                        const ERR: &str = "expected two numbers";
                        if let LoxValue::Num(lnum) = lval
                        {
                            if let LoxValue::Num(rnum) = rval
                            {
                                return match expr.oper
                                {
                                    TokenType::Minus => Ok(LoxValue::Num(lnum - rnum)),
                                    TokenType::Star => Ok(LoxValue::Num(lnum * rnum)),
                                    TokenType::Slash => Ok(LoxValue::Num(lnum / rnum)),
                                    TokenType::Percent => Ok(LoxValue::Num(lnum - (lnum / rnum).floor() * rnum)),
                                    _ => panic!()
                                };
                            }
                            else { Err((ERR, (expr.right.start(), expr.right.len()))) }
                        }
                        else { Err((ERR, (expr.left.start(), expr.left.len()))) }
                    },
                    _ => panic!()
                },
                Err(err) => Err(err)
            },
            Err(err) => Err(err)
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<LoxValue, (&'static str, (usize, usize))>
    {
        expr.expr.run(self)
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<LoxValue, (&'static str, (usize, usize))>
    {
        match expr.oper
        {
            TokenType::Bang => match expr.expr.run(self)
            {
                Ok(value) => match value
                {
                    LoxValue::Bool(value) => Ok(LoxValue::Bool(!value)),
                    LoxValue::Num(value) => Ok(LoxValue::Bool(value == 0.0)),
                    LoxValue::Str(value) => Ok(LoxValue::Bool(value.len() == 0)),
                    LoxValue::Nil => Ok(LoxValue::Bool(true))
                },
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
                    else { Err(("expected number",
                        (expr.expr.start(), expr.expr.len()))) },
                    Err(err) => Err(err)
                }
            }
            _ => panic!()
        }
    }

    fn visit_literal(&mut self, expr: &Literal) -> Result<LoxValue, (&'static str, (usize, usize))>
    {
        match &expr.value
        {
            LoxValue::Bool(value) => Ok(LoxValue::Bool(*value)),
            LoxValue::Num(value) => Ok(LoxValue::Num(*value)),
            LoxValue::Str(value) => Ok(LoxValue::Str(value.to_string())),
            LoxValue::Nil => Ok(LoxValue::Nil)
        }
    }
}
