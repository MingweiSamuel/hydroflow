use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::{Sink, SinkBuild, forward_sink};

pin_project! {
    /// Same as [`core::iterator::Map`] but as a [`Sink`].
    ///
    /// Synchronously maps items and sends the output to the following sink.
    #[must_use = "sinks do nothing unless polled"]
    pub struct Map<Si, Func> {
        #[pin]
        sink: Si,
        func: Func,
    }
}

impl<Si, Func> Map<Si, Func> {
    /// Creates with mapping `func` and next `sink`.
    pub fn new(func: Func, sink: Si) -> Self {
        Self { sink, func }
    }

    /// Creates with mapping `func` and next `sink`, ensuring this implements `Sink<Item>`.
    pub fn new_sink<Item>(func: Func, sink: Si) -> Self
    where
        Self: Sink<Item>,
    {
        Self::new(func, sink)
    }
}

impl<Si, Func, Item, ItemOut> Sink<Item> for Map<Si, Func>
where
    Si: Sink<ItemOut>,
    Func: FnMut(Item) -> ItemOut,
{
    type Error = Si::Error;

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        let this = self.project();
        let item = (this.func)(item);
        this.sink.start_send(item)
    }

    forward_sink!(poll_ready, poll_flush, poll_close);
}

/// [`SinkBuild`] for [`Map`].
pub struct MapBuilder<Prev, Func> {
    pub(crate) prev: Prev,
    pub(crate) func: Func,
}
impl<Prev, ItemOut, Func> SinkBuild for MapBuilder<Prev, Func>
where
    Prev: SinkBuild,
    Func: FnMut(Prev::Item) -> ItemOut,
{
    type Item = ItemOut;

    type Build<Next: Sink<ItemOut>> = Prev::Build<Map<Next, Func>>;

    fn build<Next>(self, next: Next) -> Self::Build<Next>
    where
        Next: Sink<ItemOut>,
    {
        self.prev.build(Map::new(self.func, next))
    }
}
