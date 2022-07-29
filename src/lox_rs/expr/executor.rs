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
                    TokenType::Less => todo!(),
                    TokenType::LessEqual => todo!(),
                    TokenType::Greater => todo!(),
                    TokenType::GreaterEqual => todo!(),
                    TokenType::Plus => todo!(),
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
