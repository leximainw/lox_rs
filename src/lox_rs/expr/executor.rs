use super::*;

pub struct AstExecutor{}
impl Visitor<Result<LoxValue, String>> for AstExecutor
{
    fn visit_binary(expr: &Binary) -> Result<LoxValue, String>
    {
        match expr.left.run()
        {
            Ok(lval) => match expr.right.run()
            {
                Ok(rval) => match expr.oper
                {
                    TokenType::EqualEqual => Ok(LoxValue::Bool(lval == rval)),
                    TokenType::BangEqual => Ok(LoxValue::Bool(lval != rval)),
                    TokenType::Less | TokenType::LessEqual
                    | TokenType::Greater | TokenType::GreaterEqual
                    | TokenType::Plus =>
                    {
                        // TODO: merge if let chaining becomes stable
                        if let LoxValue::Str(lstr) = lval
                        {
                            if let LoxValue::Str(rstr) = rval
                            {
                                return match expr.oper
                                {
                                    TokenType::Less => Ok(LoxValue::Bool(lstr < rstr)),
                                    TokenType::LessEqual => Ok(LoxValue::Bool(lstr <= rstr)),
                                    TokenType::Greater => Ok(LoxValue::Bool(lstr > rstr)),
                                    TokenType::GreaterEqual => Ok(LoxValue::Bool(lstr >= rstr)),
                                    TokenType::Plus =>
                                    {
                                        let mut cat = lstr.to_string();
                                        cat.push_str(&rstr);
                                        Ok(LoxValue::Str(cat))
                                    },
                                    _ => Err("test".to_string())
                                }
                            }
                        }
                        else if let LoxValue::Num(lnum) = lval
                        {
                            if let LoxValue::Num(rnum) = rval
                            {
                                return match expr.oper
                                {
                                    TokenType::Less => Ok(LoxValue::Bool(lnum < rnum)),
                                    TokenType::LessEqual => Ok(LoxValue::Bool(lnum <= rnum)),
                                    TokenType::Greater => Ok(LoxValue::Bool(lnum > rnum)),
                                    TokenType::GreaterEqual => Ok(LoxValue::Bool(lnum >= rnum)),
                                    TokenType::Plus => Ok(LoxValue::Num(lnum + rnum)),
                                    _ => panic!()
                                };
                            }
                        }
                        Err("expected two numbers or two strings".to_string())
                    },
                    TokenType::Minus | TokenType::Star
                    | TokenType::Slash | TokenType::Percent =>
                    {
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
                        }
                        Err("expected two numbers".to_string())
                    },
                    _ => panic!()
                },
                Err(err) => Err(err)
            },
            Err(err) => Err(err)
        }
    }

    fn visit_grouping(expr: &Grouping) -> Result<LoxValue, String>
    {
        expr.expr.run()
    }

    fn visit_unary(expr: &Unary) -> Result<LoxValue, String>
    {
        match expr.oper
        {
            TokenType::Bang => match expr.expr.run()
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
                match expr.expr.run()
                {
                    Ok(value) => if let LoxValue::Num(num) = value
                    {
                        Ok(LoxValue::Num(-num))
                    }
                    else { Err("expected number".to_string()) },
                    Err(err) => Err(err)
                }
            }
            _ => panic!()
        }
    }

    fn visit_literal(expr: &Literal) -> Result<LoxValue, String>
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
