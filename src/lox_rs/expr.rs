pub mod vm;
pub mod printer;

use super::{
    LoxValue,
    TokenType
};

// impl Visitor<String> for AstPrinter: print;
use printer::AstPrinter;

// impl Visitor<Result<LoxValue, (&'static str, (usize, usize))>> for VM: run;
use super::VM;

// trait: Expr;
// attr start: usize;
// attr len: usize;
// cast: &VarGet;

// type Binary: left: Box<dyn Expr>, oper: TokenType, right: Box<dyn Expr>;
// type Grouping: expr: Box<dyn Expr>;
// type Literal: value: LoxValue;
// type Logical: left: Box<dyn Expr>, oper: TokenType, right: Box<dyn Expr>;
// type Unary: oper: TokenType, expr: Box<dyn Expr>;
// type VarGet: name: String;
// type VarSet: name: String, expr: Box<dyn Expr>;

// autogenerated code

pub trait Expr
{
	fn start(&self) -> usize;
	fn len(&self) -> usize;
	fn as_varget(&self) -> Option<&VarGet>;

	fn print(&self, print: &mut AstPrinter) -> String;
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>;
}

trait Visitor<I>
{
	fn visit_binary(&mut self, expr: &Binary) -> I;
	fn visit_grouping(&mut self, expr: &Grouping) -> I;
	fn visit_literal(&mut self, expr: &Literal) -> I;
	fn visit_logical(&mut self, expr: &Logical) -> I;
	fn visit_unary(&mut self, expr: &Unary) -> I;
	fn visit_varget(&mut self, expr: &VarGet) -> I;
	fn visit_varset(&mut self, expr: &VarSet) -> I;
}

pub struct Binary
{
	pub start: usize,
	pub len: usize,
	pub left: Box<dyn Expr>,
	pub oper: TokenType,
	pub right: Box<dyn Expr>
}

impl Expr for Binary
{
	fn start(&self) -> usize { self.start }
	fn len(&self) -> usize { self.len }
	fn as_varget(&self) -> Option<&VarGet> { None }

	fn print(&self, print: &mut AstPrinter) -> String
	{ print.visit_binary(self) }
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>
	{ run.visit_binary(self) }
}

pub struct Grouping
{
	pub start: usize,
	pub len: usize,
	pub expr: Box<dyn Expr>
}

impl Expr for Grouping
{
	fn start(&self) -> usize { self.start }
	fn len(&self) -> usize { self.len }
	fn as_varget(&self) -> Option<&VarGet> { None }

	fn print(&self, print: &mut AstPrinter) -> String
	{ print.visit_grouping(self) }
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>
	{ run.visit_grouping(self) }
}

pub struct Literal
{
	pub start: usize,
	pub len: usize,
	pub value: LoxValue
}

impl Expr for Literal
{
	fn start(&self) -> usize { self.start }
	fn len(&self) -> usize { self.len }
	fn as_varget(&self) -> Option<&VarGet> { None }

	fn print(&self, print: &mut AstPrinter) -> String
	{ print.visit_literal(self) }
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>
	{ run.visit_literal(self) }
}

pub struct Logical
{
	pub start: usize,
	pub len: usize,
	pub left: Box<dyn Expr>,
	pub oper: TokenType,
	pub right: Box<dyn Expr>
}

impl Expr for Logical
{
	fn start(&self) -> usize { self.start }
	fn len(&self) -> usize { self.len }
	fn as_varget(&self) -> Option<&VarGet> { None }

	fn print(&self, print: &mut AstPrinter) -> String
	{ print.visit_logical(self) }
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>
	{ run.visit_logical(self) }
}

pub struct Unary
{
	pub start: usize,
	pub len: usize,
	pub oper: TokenType,
	pub expr: Box<dyn Expr>
}

impl Expr for Unary
{
	fn start(&self) -> usize { self.start }
	fn len(&self) -> usize { self.len }
	fn as_varget(&self) -> Option<&VarGet> { None }

	fn print(&self, print: &mut AstPrinter) -> String
	{ print.visit_unary(self) }
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>
	{ run.visit_unary(self) }
}

pub struct VarGet
{
	pub start: usize,
	pub len: usize,
	pub name: String
}

impl Expr for VarGet
{
	fn start(&self) -> usize { self.start }
	fn len(&self) -> usize { self.len }
	fn as_varget(&self) -> Option<&VarGet> { Some(self) }

	fn print(&self, print: &mut AstPrinter) -> String
	{ print.visit_varget(self) }
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>
	{ run.visit_varget(self) }
}

pub struct VarSet
{
	pub start: usize,
	pub len: usize,
	pub name: String,
	pub expr: Box<dyn Expr>
}

impl Expr for VarSet
{
	fn start(&self) -> usize { self.start }
	fn len(&self) -> usize { self.len }
	fn as_varget(&self) -> Option<&VarGet> { None }

	fn print(&self, print: &mut AstPrinter) -> String
	{ print.visit_varset(self) }
	fn run(&self, run: &mut VM) -> Result<LoxValue, (&'static str, (usize, usize))>
	{ run.visit_varset(self) }
}
