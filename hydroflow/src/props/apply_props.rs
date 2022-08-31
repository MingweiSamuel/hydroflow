pub struct ApplyProps<Iter> {
    iter: Iter,
}
impl<Iter> Iterator for ApplyProps<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
