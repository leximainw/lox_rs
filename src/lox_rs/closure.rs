use super::{Backtrace, LoxValue};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LoxClosure {}

impl LoxClosure {
    pub fn call(&mut self, args: Vec<LoxValue>) -> Result<LoxValue, Backtrace> {
        todo!();
    }
}
