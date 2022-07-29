use super::*;

pub struct AstPrinter{}
impl Visitor<String> for AstPrinter
{
	fn visit_binary(expr: &Binary) -> String
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
			expr.left.print(),
			expr.right.print())
	}

	fn visit_grouping(expr: &Grouping) -> String
	{
		format!("(group {})", expr.expr.print())
	}

	fn visit_literal(expr: &Literal) -> String
	{
		format!("{:?}", expr.value)
	}

	fn visit_unary(expr: &Unary) -> String
	{
		let oper = match expr.oper
		{
			TokenType::Bang => "!",
			TokenType::Minus => "-",
			_ => panic!()
		};
		format!("({oper} {})", expr.expr.print())
	}
}
