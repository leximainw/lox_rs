use std::collections::VecDeque;

pub trait NPeekableExt : Iterator + Sized
{
    fn npeekable(self) -> NPeekable<Self>;
}

impl<I: Iterator> NPeekableExt for I
{
    fn npeekable(self) -> NPeekable<I>
    {
        NPeekable{
            iter: self,
            view: VecDeque::new(),
            cursor: 0
        }
    }
}

pub struct NPeekable<I: Iterator>
{
    iter: I,
    view: VecDeque<I::Item>,
    cursor: usize
}

impl<I: Iterator> Iterator for NPeekable<I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item>
    {
        if self.view.len() != 0
        {
            self.cursor = 0;
            self.view.pop_front()
        }
        else { self.iter.next() }
    }
}

impl<I: Iterator> NPeekable<I>
{
    pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item>
    {
        match self.peek()
        {
            Some(item) if func(&item) => self.next(),
            _ => None
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item>
    {
        if self.cursor == self.view.len()
        {
            match self.iter.next()
            {
                Some(item) => self.view.push_back(item),
                None => return None
            }
        }
        Some(&self.view[self.cursor])
    }

    pub fn peek_next(&mut self) -> Option<&I::Item>
    {
        self.advance_cursor();
        self.peek()
    }

    pub fn advance_cursor(&mut self) -> bool
    {
        if self.cursor == self.view.len()
        {
            match self.iter.next()
            {
                Some(item) => self.view.push_back(item),
                None => return false
            }
        }
        self.cursor += 1;
        true
    }

    pub fn reset_cursor(&mut self)
    {
        self.cursor = 0;
    }

    pub fn unwrap(&mut self) -> &mut I
    {
        &mut self.iter
    }
}
