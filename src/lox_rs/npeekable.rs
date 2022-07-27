pub trait NPeekable : Iterator
{
    fn npeekable(self) -> ConcreteNPeekable<Self> where Self: Sized
    {
        ConcreteNPeekable{
            iter: self,
            view: Vec::new(),
            cursor: 0
        }
    }
}

pub struct ConcreteNPeekable<I: Iterator>
{
    iter: I,
    view: Vec<<I as Iterator>::Item>,
    cursor: usize
}

impl<I: Iterator> NPeekable for I {}
