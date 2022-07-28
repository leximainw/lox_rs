use super::{
    LoxValue,
    TokenType
};

pub struct AstPrinter{}
impl Visitor<String> for AstPrinter
{
	fn visit_binary(&self, expr: &Binary) -> String
	{
		let oper = match expr.oper
		{
			TokenType::Plus => "+",
			TokenType::Minus => "-",
			TokenType::Star => "*",
			TokenType::Slash => "/",
			_ => panic!()
		};
		format!("({oper} {} {})",
			expr.left.visit(self),
			expr.right.visit(self))
	}

	fn visit_grouping(&self, expr: &Grouping) -> String
	{
		format!("(group {})", expr.expr.visit(self))
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
		format!("({oper} {})", expr.expr.visit(self))
	}
}

// autogenerated code

pub trait Expr
{
	fn visit(&self, visitor: &AstPrinter) -> String;
}

trait Visitor<I>
{
	fn visit_binary(&self, expr: &Binary) -> I;
	fn visit_grouping(&self, expr: &Grouping) -> I;
	fn visit_literal(&self, expr: &Literal) -> I;
	fn visit_unary(&self, expr: &Unary) -> I;
}

pub struct Binary
{
	pub left: Box<dyn Expr>,
	pub oper: TokenType,
	pub right: Box<dyn Expr>
}

impl Expr for Binary
{
	fn visit(&self, visitor: &AstPrinter) -> String
	{
		visitor.visit_binary(self)
	}
}

pub struct Grouping
{
	pub expr: Box<dyn Expr>
}

impl Expr for Grouping
{
	fn visit(&self, visitor: &AstPrinter) -> String
	{
		visitor.visit_grouping(self)
	}
}

pub struct Literal
{
	pub value: LoxValue
}

impl Expr for Literal
{
	fn visit(&self, visitor: &AstPrinter) -> String
	{
		visitor.visit_literal(self)
	}
}

pub struct Unary
{
	pub oper: TokenType,
	pub expr: Box<dyn Expr>
}

impl Expr for Unary
{
	fn visit(&self, visitor: &AstPrinter) -> String
	{
		visitor.visit_unary(self)
	}
}
