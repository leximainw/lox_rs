use super::*;

pub struct AstPrinter{}
impl Visitor<String> for AstPrinter
{
	fn visit_binary(&self, expr: &Binary) -> String
	{
		let oper = match expr.oper
		{
			TokenType::EqualEqual => "==",
			TokenType::BangEqual => "!=",
			TokenType::Less => "<",
			TokenType::LessEqual => "<=",
			TokenType::Greater => ">",
			TokenType::GreaterEqual => ">=",
			TokenType::Plus => "+",
			TokenType::Minus => "-",
			TokenType::Star => "*",
			TokenType::Slash => "/",
			TokenType::Percent => "%",
			_ => panic!()
		};
		format!("({oper} {} {})",
			expr.left.print(self),
			expr.right.print(self))
	}

	fn visit_grouping(&self, expr: &Grouping) -> String
	{
		format!("(group {})", expr.expr.print(self))
	}

	fn visit_literal(&self, expr: &Literal) -> String
	{
		format!("{:?}", expr.value)
	}

	fn visit_unary(&self, expr: &Unary) -> String
	{
		let oper = match expr.oper
		{
			TokenType::Bang => "!",
			TokenType::Minus => "-",
			_ => panic!()
		};
		format!("({oper} {})", expr.expr.print(self))
	}
}
