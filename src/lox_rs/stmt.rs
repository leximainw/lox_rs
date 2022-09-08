pub mod vm;

use super::expr::Expr;

// impl Visitor<Result<(), Backtrace>> for VM: run;
use super::{Backtrace, VM};

// trait: Stmt;
// attr start: usize;
// attr len: usize;
// cast: ExprStmt;

// type BlockStmt: stmts: Vec<Box<dyn Stmt>>;
// type ExprStmt: expr: Box<dyn Expr>;
// type IfStmt: expr: Box<dyn Expr>, stmt_true: Box<dyn Stmt>, stmt_false: Option<Box<dyn Stmt>>;
// type PrintStmt: expr: Box<dyn Expr>;
// type VarStmt: name: String, expr: Option<Box<dyn Expr>>;
// type WhileStmt: expr: Box<dyn Expr>, stmt: Box<dyn Stmt>;

// autogenerated code

pub trait Stmt
{
    fn start(&self) -> usize;
    fn len(&self) -> usize;
    fn to_exprstmt(self: Box<Self>) -> Option<ExprStmt>;

    fn run(&self, run: &mut VM) -> Result<(), Backtrace>;
}

trait Visitor<I>
{
    fn visit_blockstmt(&mut self, expr: &BlockStmt) -> I;
    fn visit_exprstmt(&mut self, expr: &ExprStmt) -> I;
    fn visit_ifstmt(&mut self, expr: &IfStmt) -> I;
    fn visit_printstmt(&mut self, expr: &PrintStmt) -> I;
    fn visit_varstmt(&mut self, expr: &VarStmt) -> I;
    fn visit_whilestmt(&mut self, expr: &WhileStmt) -> I;
}

pub struct BlockStmt
{
    pub start: usize,
    pub len: usize,
    pub stmts: Vec<Box<dyn Stmt>>
}

impl Stmt for BlockStmt
{
    fn start(&self) -> usize { self.start }
    fn len(&self) -> usize { self.len }
    fn to_exprstmt(self: Box<Self>) -> Option<ExprStmt> { None }

    fn run(&self, run: &mut VM) -> Result<(), Backtrace>
    { run.visit_blockstmt(self) }
}

pub struct ExprStmt
{
    pub start: usize,
    pub len: usize,
    pub expr: Box<dyn Expr>
}

impl Stmt for ExprStmt
{
    fn start(&self) -> usize { self.start }
    fn len(&self) -> usize { self.len }
    fn to_exprstmt(self: Box<Self>) -> Option<ExprStmt> { Some(*self) }

    fn run(&self, run: &mut VM) -> Result<(), Backtrace>
    { run.visit_exprstmt(self) }
}

pub struct IfStmt
{
    pub start: usize,
    pub len: usize,
    pub expr: Box<dyn Expr>,
    pub stmt_true: Box<dyn Stmt>,
    pub stmt_false: Option<Box<dyn Stmt>>
}

impl Stmt for IfStmt
{
    fn start(&self) -> usize { self.start }
    fn len(&self) -> usize { self.len }
    fn to_exprstmt(self: Box<Self>) -> Option<ExprStmt> { None }

    fn run(&self, run: &mut VM) -> Result<(), Backtrace>
    { run.visit_ifstmt(self) }
}

pub struct PrintStmt
{
    pub start: usize,
    pub len: usize,
    pub expr: Box<dyn Expr>
}

impl Stmt for PrintStmt
{
    fn start(&self) -> usize { self.start }
    fn len(&self) -> usize { self.len }
    fn to_exprstmt(self: Box<Self>) -> Option<ExprStmt> { None }

    fn run(&self, run: &mut VM) -> Result<(), Backtrace>
    { run.visit_printstmt(self) }
}

pub struct VarStmt
{
    pub start: usize,
    pub len: usize,
    pub name: String,
    pub expr: Option<Box<dyn Expr>>
}

impl Stmt for VarStmt
{
    fn start(&self) -> usize { self.start }
    fn len(&self) -> usize { self.len }
    fn to_exprstmt(self: Box<Self>) -> Option<ExprStmt> { None }

    fn run(&self, run: &mut VM) -> Result<(), Backtrace>
    { run.visit_varstmt(self) }
}

pub struct WhileStmt
{
    pub start: usize,
    pub len: usize,
    pub expr: Box<dyn Expr>,
    pub stmt: Box<dyn Stmt>
}

impl Stmt for WhileStmt
{
    fn start(&self) -> usize { self.start }
    fn len(&self) -> usize { self.len }
    fn to_exprstmt(self: Box<Self>) -> Option<ExprStmt> { None }

    fn run(&self, run: &mut VM) -> Result<(), Backtrace>
    { run.visit_whilestmt(self) }
}
