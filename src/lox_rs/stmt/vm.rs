use super::{
    *,
    super::{
        LoxValue,
        VM
    }
};

impl Visitor<Result<(), Backtrace>> for VM
{
    fn visit_blockstmt(&mut self, block: &BlockStmt) -> Result<(), Backtrace>
    {
        self.new_scope();
        for stmt in &block.stmts
        {
            match stmt.run(self)
            {
                Ok(_) => {},
                Err(err) =>
                {
                    self.unscope();
                    return Err(err);
                }
            }
        }
        self.unscope();
        return Ok(());
    }

    fn visit_exprstmt(&mut self, stmt: &ExprStmt) -> Result<(), Backtrace>
    {
        match stmt.expr.run(self)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn visit_ifstmt(&mut self, stmt: &IfStmt) -> Result<(), Backtrace>
    {
        match stmt.expr.run(self)
        {
            Ok(value) => {
                if LoxValue::is_truthy(&value)
                {
                    stmt.stmt_true.run(self)
                }
                else if let Some(stmt) = &stmt.stmt_false
                {
                    stmt.run(self)
                }
                else { Ok(()) }
            },
            Err(err) => Err(err)
        }
    }

    fn visit_printstmt(&mut self, stmt: &PrintStmt) -> Result<(), Backtrace>
    {
        match stmt.expr.run(self)
        {
            Ok(value) => { println!("{value}"); Ok(()) },
            Err(err) => Err(err)
        }
    }

    fn visit_varstmt(&mut self, stmt: &VarStmt) -> Result<(), Backtrace>
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

    fn visit_whilestmt(&mut self, stmt: &WhileStmt) -> Result<(), Backtrace>
    {
        loop
        {
            match stmt.expr.run(self)
            {
                Ok(value) =>
                {
                    if !LoxValue::is_truthy(&value)
                    {
                        return Ok(())
                    }
                    match stmt.stmt.run(self)
                    {
                        Err(err) => return Err(err),
                        _ => {}
                    }
                },
                Err(err) => return Err(err)
            }
        }
    }
}
