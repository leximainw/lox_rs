use super::*;

pub struct AstExecutor{}
impl Visitor<Result<(), (&'static str, (usize, usize))>> for AstExecutor
{
    fn visit_exprstmt(stmt: &ExprStmt) -> Result<(), (&'static str, (usize, usize))>
    {
        match stmt.expr.run()
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn visit_printstmt(stmt: &PrintStmt) -> Result<(), (&'static str, (usize, usize))>
    {
        match stmt.expr.run()
        {
            Ok(value) => { println!("{value}"); Ok(()) },
            Err(err) => Err(err)
        }
    }
}
