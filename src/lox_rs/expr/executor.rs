use super::*;

pub struct AstExecutor{}
impl Visitor<Result<LoxValue, String>> for AstExecutor
{
    fn visit_binary(&self, expr: &Binary) -> Result<LoxValue, String>
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
                    | TokenType::Greater | TokenType::GreaterEqual
                    | TokenType::Plus =>
                    {
                        // TODO: merge if let chaining becomes stable
                        if let LoxValue::Str(lstr) = lval
                        {
                            if let LoxValue::Str(rstr) = rval
                            {
                                return match(expr.oper)
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
                                return match(expr.oper)
                                {
                                    TokenType::Less => Ok(LoxValue::Bool(lnum < rnum)),
                                    TokenType::LessEqual => Ok(LoxValue::Bool(lnum <= rnum)),
                                    TokenType::Greater => Ok(LoxValue::Bool(lnum > rnum)),
                                    TokenType::GreaterEqual => Ok(LoxValue::Bool(lnum >= rnum)),
                                    TokenType::Plus => Ok(LoxValue::Num(lnum + rnum)),
                                    _ => Err("test".to_string())
                                };
                            }
                        }
                        Err("test".to_string())
                    },
                    TokenType::Minus => todo!(),
                    TokenType::Star => todo!(),
                    TokenType::Slash => todo!(),
                    TokenType::Percent => todo!(),
                    _ => panic!()
                },
                Err(err) => Err(err)
            },
            Err(err) => Err(err)
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Result<LoxValue, String>
    {
        expr.run(&self)
    }

    fn visit_unary(&self, expr: &Unary) -> Result<LoxValue, String>
    {
        match expr.oper
        {
            TokenType::Bang => todo!(),
            TokenType::Minus => todo!(),
            _ => panic!()
        }
    }

    fn visit_literal(&self, expr: &Literal) -> Result<LoxValue, String>
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

impl AstExecutor
{
    fn get_value_type(value: &LoxValue) -> TokenType
    {
        match value
        {
            LoxValue::Bool(_) => TokenType::Boolean,
            LoxValue::Num(_) => TokenType::Number,
            LoxValue::Str(_) => TokenType::String,
            LoxValue::Nil => TokenType::Nil
        }
    }
}
