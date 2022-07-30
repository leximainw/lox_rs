use super::*;

pub struct AstExecutor{}
impl Visitor<Result<(), (&'static str, (usize, usize))>> for AstExecutor
{
    fn visit_exprstmt(stmt: &ExprStmt) -> Result<(), (&'static str, (usize, usize))>
    {
        Ok(())
    }

    fn visit_printstmt(stmt: &PrintStmt) -> Result<(), (&'static str, (usize, usize))>
    {
        Ok(())
    }
}
