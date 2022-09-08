use super::LoxValue;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct LoxClosure
{

}

impl LoxClosure
{
    pub fn call(&mut self, args: Vec<LoxValue>) -> Result<LoxValue, (&'static str, (usize, usize))>
    {
        todo!();
    }
}
