use super::super::VM;
use super::*;

impl Visitor<Result<(), (&'static str, (usize, usize))>> for VM
{
    fn visit_exprstmt(&mut self, stmt: &ExprStmt) -> Result<(), (&'static str, (usize, usize))>
    {
        match stmt.expr.run(self)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn visit_printstmt(&mut self, stmt: &PrintStmt) -> Result<(), (&'static str, (usize, usize))>
    {
        match stmt.expr.run(self)
        {
            Ok(value) => { println!("{value}"); Ok(()) },
            Err(err) => Err(err)
        }
    }

    fn visit_varstmt(&mut self, stmt: &VarStmt) -> Result<(), (&'static str, (usize, usize))>
    {
        todo!();
    }
}
