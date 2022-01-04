use crate::compiled::{Pusherator, PusheratorBuild};
use std::marker::PhantomData;

// This is needed because of this:
// https://github.com/rust-lang/rust/issues/87479#issue-952936723
pub trait IteratorFactorySuper {
    type Item;
    type Iterator<'i>: Iterator<Item = Self::Item>;
}

pub trait IteratorFactory: IteratorFactorySuper {
    type Input;
    fn build_iterator(&mut self, input: Self::Input) -> Self::Iterator<'_>;

    fn map<B, F>(self, f: F) -> MapIteratorFactory<Self, F, B>
    where
        F: FnMut(Self::Item) -> B,
        B: Iterator,
        Self: Sized,
    {
        MapIteratorFactory {
            prev: self,
            func: f,
            _phantom: PhantomData,
        }
    }

    fn flat_map<B, F>(self, f: F) -> FlatMapIteratorFactory<Self, F, B>
    where
        F: FnMut(Self::Item) -> B,
        B: Iterator,
        B::Item: IntoIterator,
        Self: Sized,
    {
        FlatMapIteratorFactory {
            prev: self,
            func: f,
            _phantom: PhantomData,
        }
    }

    fn filter<F>(self, f: F) -> FilterIteratorFactory<Self, F>
    where
        F: FnMut(&Self::Item) -> bool,
        Self: Sized,
    {
        FilterIteratorFactory {
            prev: self,
            func: f,
        }
    }

    fn pusherator(&mut self) -> PusheratorFactory<'_, Self> {
        PusheratorFactory { iter: self }
    }
}

pub struct PusheratorFactory<'a, I>
where
    I: 'static + ?Sized + IteratorFactory,
{
    iter: &'a mut I,
}
impl<'a, I> PusheratorBuild for PusheratorFactory<'a, I>
where
    I: 'static + ?Sized + IteratorFactory,
{
    type Item = I::Item;

    type Output<O: Pusherator<Item = Self::Item>> = PivotBuild<'a, I, O>;
    fn build<O>(self, input: O) -> Self::Output<O>
    where
        O: Pusherator<Item = Self::Item>,
    {
        PivotBuild {
            pull: self.iter,
            push: input,
        }
    }
}

type MapIteratorFactoryIterator<'i, I, F, Out> = impl 'i + Iterator<Item = Out>;
pub struct MapIteratorFactory<I, F, Out>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(I::Item) -> Out,
    Out: 'static,
{
    prev: I,
    func: F,
    _phantom: PhantomData<fn(I::Item) -> Out>,
}
impl<I, F, Out> IteratorFactorySuper for MapIteratorFactory<I, F, Out>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(I::Item) -> Out,
    Out: 'static,
{
    type Item = Out;
    type Iterator<'i> = MapIteratorFactoryIterator<'i, I, F, Out>;
}
impl<I, F, Out> IteratorFactory for MapIteratorFactory<I, F, Out>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(I::Item) -> Out,
    Out: 'static,
{
    type Input = I::Input;
    fn build_iterator(&mut self, input: Self::Input) -> Self::Iterator<'_> {
        self.prev.build_iterator(input).map(|x| (self.func)(x))
    }
}

type FlatMapIteratorFactoryIterator<'i, I, F, Out> =
    impl 'i + Iterator<Item = <Out as IntoIterator>::Item>;
pub struct FlatMapIteratorFactory<I, F, Out>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(I::Item) -> Out,
    Out: 'static + IntoIterator,
{
    prev: I,
    func: F,
    _phantom: PhantomData<fn(I::Item) -> Out>,
}
impl<I, F, Out> IteratorFactorySuper for FlatMapIteratorFactory<I, F, Out>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(I::Item) -> Out,
    Out: 'static + IntoIterator,
{
    type Item = Out::Item;
    type Iterator<'i> = FlatMapIteratorFactoryIterator<'i, I, F, Out>;
}
impl<I, F, Out> IteratorFactory for FlatMapIteratorFactory<I, F, Out>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(I::Item) -> Out,
    Out: 'static + IntoIterator,
{
    type Input = I::Input;
    fn build_iterator(&mut self, input: Self::Input) -> Self::Iterator<'_> {
        self.prev.build_iterator(input).flat_map(|x| (self.func)(x))
    }
}

type FilterIteratorFactoryIterator<'i, I, F> =
    impl 'i + Iterator<Item = <I as IteratorFactorySuper>::Item>;
pub struct FilterIteratorFactory<I, F>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(&I::Item) -> bool,
{
    prev: I,
    func: F,
}
impl<I, F> IteratorFactorySuper for FilterIteratorFactory<I, F>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    type Iterator<'i> = FilterIteratorFactoryIterator<'i, I, F>;
}
impl<I, F> IteratorFactory for FilterIteratorFactory<I, F>
where
    I: 'static + IteratorFactory,
    F: 'static + FnMut(&I::Item) -> bool,
{
    type Input = I::Input;
    fn build_iterator(&mut self, input: Self::Input) -> Self::Iterator<'_> {
        self.prev.build_iterator(input).filter(|x| (self.func)(x))
    }
}

#[allow(dead_code)] // TODO(mingwei)
pub struct PivotBuild<'a, I, O>
where
    I: 'static + ?Sized + IteratorFactory,
    O: Pusherator<Item = I::Item>,
{
    pull: &'a mut I,
    push: O,
}
impl<'a, I, O> PivotBuild<'a, I, O>
where
    I: 'static + ?Sized + IteratorFactory,
    O: Pusherator<Item = I::Item>,
{
}
