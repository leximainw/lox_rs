pub struct NPeekable<I: Iterator>
{
    iter: I,
    view: Vec<<I as Iterator>::Item>,
    cursor: usize
}

impl<I: Iterator> Iterator for NPeekable<I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item>
    {
        self.iter.next()
    }
}