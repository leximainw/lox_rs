pub trait NPeekable : Iterator + Sized
{
    fn npeekable(self) -> ConcreteNPeekable<Self>;
}

impl<I: Iterator> NPeekable for I
{
    fn npeekable(self) -> ConcreteNPeekable<I>
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

impl<I: Iterator> Iterator for ConcreteNPeekable<I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item>
    {
        self.iter.next()
    }
}
