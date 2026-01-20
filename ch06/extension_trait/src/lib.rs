pub trait ReverseExt<T> {
    fn reversed(&self) -> Vec<T>;
}

impl<T> ReverseExt<T> for Vec<T>
where
    T: Clone,
{
    fn reversed(&self) -> Vec<T> {
        self.iter().rev().cloned().collect()
    }
}

pub trait DoubleEndedIteratorExt: DoubleEndedIterator {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>;
}

impl<I: DoubleEndedIterator> DoubleEndedIteratorExt for I {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>,
    {
        self.rev().cloned().collect()
    }
}
