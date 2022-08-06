use std::collections::HashMap;
use std::mem::swap;

use super::{
    Errors,
    Parser,
    LoxValue
};

pub struct VM
{
    pub curr_scope: Scope
}

impl VM
{
    pub fn new() -> VM
    {
        VM{
            curr_scope: Scope::new()
        }
    }

    pub fn run(&mut self, code: &str)
    {
        let mut errors: Errors = Errors::new(code);
        let mut parser: Parser = Parser::new(code);
        while let Some(stmt) = parser.next()
        {
            match stmt.run(self)
            {
                Ok(()) => {},
                Err(err) =>
                {
                    let (msg, pos) = err;
                    let (start, len) = pos;
                    Self::print_error(code, "Runtime", msg, start, len);
                }
            }
        }
        parser.coalesce_errors(&mut errors);
        errors.print_errors(Box::new(|code, msg, sev, start, len| Self::print_error(code, msg, sev, start, len)));
    }

    pub fn new_scope(&mut self)
    {
        let mut scope = Scope::new();
        swap(&mut scope, &mut self.curr_scope);
        self.curr_scope = Scope::new_inner(scope);
    }

    pub fn unscope(&mut self)
    {
        let mut scope = Scope::new();
        swap(&mut scope, &mut self.curr_scope);
        if let Some(scope) = scope.unscope()
        {
            self.curr_scope = *scope;
        }
    }

    fn print_error(code: &str, sev: &str, msg: &str, start: usize, len: usize)
    {
        let mut index = 0;
        let mut line = 1;
        let mut line_start = 0;
        let mut line_next = 0;
        loop
        {
            if let Some(needle) = code[index..]
                .find('\n').map(|i| i + index)
            {
                if index > start { break }
                line_start = line_next;
                line_next = needle + 1;
                index = line_next;
                line += 1;
            }
            else
            {
                line_start = line_next;
                line_next = code.len() + 1;
                break;
            }
        }
        println!("{sev}: {msg}");
        let line_prefix = format!("line {line}: ");
        println!("{line_prefix}{}", &code[line_start .. line_next - 1]);
        if len != 0
        {
            println!("{}{}", format!("{:>1$}", "here --", start + line_prefix.len()),
                format!("{:^<1$}", "", len));
        }
        else if start < (line_start + line_next - 1) / 2
        {
            println!("{}\\__ here", format!("{:>1$}", "", start + line_prefix.len()));
        }
        else
        {
            println!("{}", format!("{:>1$}", "here __/", start + line_prefix.len()));
        }
    }
}

pub struct Scope
{
    pub vars: HashMap<String, LoxValue>,
    pub outer: Option<Box<Scope>>
}

impl Scope
{
    pub fn new() -> Scope
    {
        Scope{
            vars: HashMap::new(),
            outer: None
        }
    }

    pub fn new_inner(outer: Scope) -> Scope
    {
        Scope{
            vars: HashMap::new(),
            outer: Some(Box::new(outer))
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue)
    {
        self.vars.insert(name, value);
    }

    pub fn get(&mut self, name: &String) -> Option<&LoxValue>
    {
        if let Some(value) = self.vars.get(name)
        {
            Some(value)
        }
        else if let Some(outer) = self.outer.as_mut()
        {
            outer.get(name)
        }
        else
        {
            None
        }
    }

    pub fn set(&mut self, name: String, value: LoxValue) -> bool
    {
        if self.vars.contains_key(&name)
        {
            self.vars.insert(name, value);
            true
        }
        else if let Some(outer) = self.outer.as_mut()
        {
            outer.set(name, value)
        }
        else
        {
            false
        }
    }

    pub fn unscope(self) -> Option<Box<Scope>>
    {
        self.outer
    }
}
