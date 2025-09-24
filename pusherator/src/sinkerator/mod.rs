use std::pin::Pin;
use std::task::{Context, Poll};

/// A `Sinkerator` is a value into which other values can be sent, asynchronously.
///
/// Provides the same functionality as [`futures::Sink`] but with a slightly
/// simplified API, which avoids the "pre-flight" polling of [`futures::Sink::poll_ready`].
pub trait Sinkerator<Item> {
    /// The type of value produced by the sink when an error occurs.
    type Error;

    /// Sends an item to the sink.
    ///
    /// If this method returns `Poll::Pending`, then the previous item has not been fully handled
    /// and this method must be called again with `None` before a new `Some` value may be sent.
    ///
    /// In most cases, if the sink encounters an error, the sink will permanently be unable to
    /// receive items.
    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        item: Option<Item>,
    ) -> Poll<Result<(), Self::Error>>;

    /// Flush any remaining output from this sink.
    ///
    /// Returns `Poll::Ready(Ok(()))` when no buffered items remain. If this
    /// value is returned then it is guaranteed that all previous values sent
    /// via `start_send` have been flushed.
    ///
    /// Returns `Poll::Pending` if there is more work left to do, in which
    /// case the current task is scheduled (via `cx.waker().wake_by_ref()`) to wake up when
    /// `poll_flush` should be called again.
    ///
    /// In most cases, if the sink encounters an error, the sink will
    /// permanently be unable to receive items.
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    /// Flush any remaining output and close this sink, if necessary.
    ///
    /// Returns `Poll::Ready(Ok(()))` when no buffered items remain and the sink
    /// has been successfully closed.
    ///
    /// Returns `Poll::Pending` if there is more work left to do, in which
    /// case the current task is scheduled (via `cx.waker().wake_by_ref()`) to wake up when
    /// `poll_close` should be called again.
    ///
    /// If this function encounters an error, the sink should be considered to
    /// have failed permanently, and no more `Sink` methods should be called.
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

mod filter;
mod flatten;
mod for_each;
mod map;
mod sink_compat;
pub use filter::Filter;
pub use flatten::Flatten;
pub use for_each::ForEach;
pub use map::Map;
pub use sink_compat::SinkCompat;
