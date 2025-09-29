#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

use core::marker::PhantomData;

pub use either;
pub use futures_util::sink;
pub use futures_util::sink::Sink;
#[cfg(feature = "variadics")]
#[cfg_attr(docsrs, doc(cfg(feature = "variadics")))]
pub use variadics;

pub mod filter;
pub mod filter_map;
pub mod flat_map;
pub mod flatten;
pub mod for_each;
pub mod inspect;
pub mod map;
pub mod send_all_iter;
pub mod try_for_each;
pub mod unzip;
pub use filter::Filter;
pub use filter_map::FilterMap;
pub use flat_map::FlatMap;
pub use flatten::Flatten;
pub use for_each::ForEach;
pub use inspect::Inspect;
pub use map::Map;
pub use send_all_iter::SendAllIter;
pub use try_for_each::TryForEach;
pub use unzip::Unzip;

#[cfg(feature = "variadics")]
#[cfg_attr(docsrs, doc(cfg(feature = "variadics")))]
mod demux_var;
#[cfg(feature = "variadics")]
#[cfg_attr(docsrs, doc(cfg(feature = "variadics")))]
pub use demux_var::{DemuxVar, SinkVariadic};

/// A [`Sink`] blanket implementation that provides extra adaptors and methods.
pub trait Sinktools<Item>: Sink<Item> {
    /// Creates a "by reference" adapter for this sink.
    ///
    /// This allows you to use this sink in functions that consume `Sink` without actually consuming this underlying sink.
    fn by_ref(&mut self) -> &mut Self {
        self
    }

    /// A futures that completes after processing all items from the iterator into this sink, then flushing.
    ///
    /// Equivalent to:
    /// ```rust,ignore
    /// async fn send_all_iter<I>(self, into_iter: I) -> Result<(), Self::Error>;
    /// ```
    ///
    /// This consumes both the iterator and the sink. Use [`Iterator::by_ref`] or [`Sinktools::by_ref`] respectively to avoid consuming them.
    fn send_all_iter<I>(self, into_iter: I) -> SendAllIter<I::IntoIter, Self>
    where
        Self: Sized,
        I: IntoIterator<Item = Item>,
    {
        SendAllIter::new(into_iter.into_iter(), self)
    }

    /// Combines this sink with another to create a fanout. Each item passed into the fanout will be cloned and pushed to both this and the other sink.
    fn un_fanout<SiOther>(self, other: SiOther) -> sink::Fanout<Self, SiOther>
    where
        Self: Sized,
        Item: Clone,
        SiOther: Sink<Item, Error = Self::Error>,
    {
        sink::SinkExt::fanout(self, other)
    }

    /// Adds a function which both filters and maps, _then_ passes the outputs into this sink.
    ///
    /// Note this places the filter_map _before_ this sink, unlike [`Iterator::filter_map`] which goes _after_.
    fn un_filter_map<In, Func>(self, func: Func) -> FilterMap<Self, Func>
    where
        Self: Sized,
        Func: FnMut(In) -> Option<Item>,
    {
        FilterMap::new_sink(func, self)
    }

    /// Adds a predicate function which is called on each element, which determines if each item may _then_ be passed into this sink.
    ///
    /// Note this places the filter _before_ this sink, unlike [`Iterator::filter`] which goes _after_.
    fn un_filter<Func>(self, func: Func) -> Filter<Self, Func>
    where
        Self: Sized,
        Func: FnMut(&Item) -> bool,
    {
        Filter::new_sink(func, self)
    }

    /// Adds a function which is called on each element, _then_ iterates and passes each output item into this sink.
    ///
    /// Note this places the flat_map _before_ this sink, unlike [`Iterator::flat_map`] which goes _after_.
    fn un_flat_map<In, Func, IntoIter>(
        self,
        func: Func,
    ) -> FlatMap<Self, Func, IntoIter::IntoIter, Item>
    where
        Self: Sized,
        Func: FnMut(In) -> IntoIter,
        IntoIter: IntoIterator<Item = Item>,
    {
        FlatMap::new_sink(func, self)
    }

    /// Takes in an iterable element, _then_ iterates and passes each output item into this sink.
    ///
    /// Note this places the flatten _before_ this sink, unlike [`Iterator::flatten`] which goes _after_.
    fn un_flatten<In>(self) -> Flatten<Self, In::IntoIter, Item>
    where
        Self: Sized,
        In: IntoIterator<Item = Item>,
    {
        Flatten::new_sink::<In>(self)
    }

    /// Does something with each element, _then_ passes each item into this sink.
    ///
    /// Note this places the inspect _before_ this sink, unlike [`Iterator::inspect`] which goes _after_.
    fn un_inspect<Func>(self, func: Func) -> Inspect<Self, Func>
    where
        Self: Sized,
        Func: FnMut(&Item),
    {
        Inspect::new_sink(func, self)
    }

    /// Adds a function which is called on each element, _then_ then each output is passed into this sink.
    ///
    /// Note this places the map _before_ this sink, unlike [`Iterator::map`] which goes _after_.
    fn un_map<In, Func>(self, func: Func) -> Map<Self, Func>
    where
        Self: Sized,
        Func: FnMut(In) -> Item,
    {
        Map::new_sink(func, self)
    }

    /// Combines this sink with another to create an unzip. Each `(Item, ItemOther)` passed into the unzip will be
    /// split and have `Item` passed to this sink, and `ItemOther` passed to the `other` argument sink.
    fn un_unzip<SiOther, ItemOther>(self, other: SiOther) -> Unzip<Self, SiOther>
    where
        Self: Sized,
        SiOther: Sink<ItemOther>,
    {
        Unzip::new_sink(self, other)
    }
}
impl<Si, Item> Sinktools<Item> for Si where Si: Sink<Item> {}

/// A helper trait for building [`Sink`]s in forward order, unlike with `Sinktools`.
///
/// To start a sink adaptor chain, use [`SinkBuilder`].
pub trait SinkBuild {
    /// The output item type.
    type Item;

    /// The built [`Sink`] type, if it is prepended to `Next`.
    type Build<Next: Sink<Self::Item>>;
    /// Prepend this sink to `next`.
    fn build<Next>(self, next: Next) -> Self::Build<Next>
    where
        Next: Sink<Self::Item>;

    /// Clone each item and send to both `sink0` and `sink1`, completing this sink adaptor chain.
    fn fanout<Si0, Si1>(self, sink0: Si0, sink1: Si1) -> Self::Build<sink::Fanout<Si0, Si1>>
    where
        Self: Sized,
        Self::Item: Clone,
        Si0: Sink<Self::Item>,
        Si1: Sink<Self::Item, Error = Si0::Error>,
    {
        self.build(sink::SinkExt::fanout(sink0, sink1))
    }

    /// Appends a function which consumes each element, completing this sink adaptor chain.
    fn for_each<Func>(self, func: Func) -> Self::Build<ForEach<Func>>
    where
        Self: Sized,
        Func: FnMut(Self::Item),
    {
        self.build(ForEach::new(func))
    }

    /// Appends a function which consumes each element and returns a result, completing this sink
    /// adaptor chain.
    fn try_for_each<Func, Error>(self, func: Func) -> Self::Build<TryForEach<Func>>
    where
        Self: Sized,
        Func: FnMut(Self::Item) -> Result<(), Error>,
    {
        self.build(TryForEach::new(func))
    }

    /// Appends a function which is called on each element and pases along each output.
    fn map<Func, Out>(self, func: Func) -> map::MapBuilder<Self, Func>
    where
        Self: Sized,
        Func: FnMut(Self::Item) -> Out,
    {
        map::MapBuilder { prev: self, func }
    }

    /// Appends a predicate function which filters items.
    fn filter<Func>(self, func: Func) -> filter::FilterBuilder<Self, Func>
    where
        Self: Sized,
        Func: FnMut(&Self::Item) -> bool,
    {
        filter::FilterBuilder { prev: self, func }
    }

    /// Appends a function which both filters and maps items.
    fn filter_map<Func, Out>(self, func: Func) -> filter_map::FilterMapBuilder<Self, Func>
    where
        Self: Sized,
        Func: FnMut(Self::Item) -> Option<Out>,
    {
        filter_map::FilterMapBuilder { prev: self, func }
    }

    /// Appends a function which maps each item to an iterator and flattens the results.
    fn flat_map<Func, IntoIter>(self, func: Func) -> flat_map::FlatMapBuilder<Self, Func>
    where
        Self: Sized,
        Func: FnMut(Self::Item) -> IntoIter,
        IntoIter: IntoIterator,
    {
        flat_map::FlatMapBuilder { prev: self, func }
    }

    /// Flattens items that are iterators.
    fn flatten<IntoIter>(self) -> flatten::FlattenBuilder<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten::FlattenBuilder { prev: self }
    }

    /// Appends a function which inspects each item without modifying it.
    fn inspect<Func>(self, func: Func) -> inspect::InspectBuilder<Self, Func>
    where
        Self: Sized,
        Func: FnMut(&Self::Item),
    {
        inspect::InspectBuilder { prev: self, func }
    }

    /// Splits items into two sinks based on tuple structure.
    fn unzip<Si0, Si1, Item0, Item1>(
        self,
        sink0: Si0,
        sink1: Si1,
    ) -> Self::Build<Map<Unzip<Si0, Si1>, fn(Self::Item) -> (Item0, Item1)>>
    where
        Self: Sized,
        Self::Item: Into<(Item0, Item1)>,
        Si0: Sink<Item0>,
        Si1: Sink<Item1>,
    {
        self.build(Map::new(|item| item.into(), Unzip::new(sink0, sink1)))
    }
}

/// Start a [`SinkBuild`] adaptor chain, with `Item` as the input item type.
pub struct SinkBuilder<Item>(PhantomData<fn() -> Item>);
impl<Item> Default for SinkBuilder<Item> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<Item> SinkBuilder<Item> {
    /// Create a new sink builder.
    pub fn new() -> Self {
        Self::default()
    }
}
impl<Item> SinkBuild for SinkBuilder<Item> {
    type Item = Item;

    type Build<Next: Sink<Self::Item>> = Next;
    fn build<Next>(self, next: Next) -> Self::Build<Next>
    where
        Next: Sink<Self::Item>,
    {
        next
    }
}

macro_rules! forward_sink {
    (
        $( $method:ident ),+
    ) => {
        $(
            fn $method(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
                self.project().sink.$method(cx)
            }
        )+
    }
}
use forward_sink;

macro_rules! ready_both {
    ($a:expr, $b:expr $(,)?) => {
        if !matches!(
            ($a, $b),
            (::core::task::Poll::Ready(()), ::core::task::Poll::Ready(())),
        ) {
            return ::core::task::Poll::Pending;
        }
    };
}
use ready_both;
