//! A chaining iterator. It allows you to chain arbitrary number of same type iterators at run time.

use std::collections::VecDeque;

/// A chain of iterators with type I.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IterChain<I> {
    iters: VecDeque<I>,
}

impl<I> IterChain<I> {
    pub fn new() -> IterChain<I>
    where
        I: Iterator,
    {
        IterChain {
            iters: VecDeque::new(),
        }
    }

    /// Include the given iterator at the end of the chain.
    pub fn include(&mut self, new_iter: I)
    where
        I: Iterator,
    {
        self.iters.push_back(new_iter);
    }

    /// Include the given iterator at the front of the chain.
    ///
    /// ```
    /// let mut i = chaining_iter::IterChain::new();
    /// i.include(3..5);
    /// i.include_front(0..3);
    ///
    /// assert_eq!(Some(0), i.next());
    /// ```
    pub fn include_front(&mut self, new_iter: I)
    where
        I: Iterator,
    {
        self.iters.push_front(new_iter);
    }
}

impl<I> Iterator for IterChain<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(iter) = self.iters.front_mut() {
                let val = iter.next();
                if val.is_some() {
                    return val;
                } else {
                    self.iters.pop_front();
                }
            } else {
                return None;
            }
        }
    }
}

impl<I> DoubleEndedIterator for IterChain<I>
where
    I: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(iter) = self.iters.back_mut() {
                let val = iter.next_back();
                if val.is_some() {
                    return val;
                } else {
                    self.iters.pop_back();
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    #[test]
    fn empty() {
        let mut i: IterChain<Range<usize>> = IterChain::new();

        assert_eq!(None, i.next());
    }

    #[test]
    fn contains_all_empty() {
        let mut i = IterChain::new();
        i.include(0..0);
        i.include(0..0);

        assert_eq!(None, i.next());
    }

    #[test]
    fn starts_with_empty() {
        let mut i = IterChain::new();
        i.include(0..0);
        i.include(1..2);

        assert_eq!(Some(1), i.next());
        assert_eq!(None, i.next());
    }

    #[test]
    fn empty_in_middle() {
        let mut i = IterChain::new();
        i.include(0..1);
        i.include(1..1);
        i.include(2..3);

        assert_eq!(Some(0), i.next());
        assert_eq!(Some(2), i.next());
        assert_eq!(None, i.next());
    }

    #[test]
    fn double_ended_iter() {
        let mut i = IterChain::new();

        i.include(0..3);
        i.include(5..7);

        assert_eq!(Some(0), i.next());
        assert_eq!(Some(6), i.next_back());
    }
}
