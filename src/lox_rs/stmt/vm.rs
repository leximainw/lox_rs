use super::{
    *,
    super::{
        LoxValue,
        VM
    }
};

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
        if let Some(expr) = &stmt.expr
        {
            match expr.run(self)
            {
                Ok(value) =>
                {
                    self.curr_scope.define(stmt.name.to_string(), value);
                    Ok(())
                },
                Err(err) => Err(err)
            }
        }
        else
        {
            self.curr_scope.define(stmt.name.to_string(), LoxValue::Nil);
            Ok(())
        }
    }
}
