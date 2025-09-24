use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;

use super::Sinkerator;

pin_project! {
    /// Compatability wrapper to allow a `Sink` to be used as a `Sinkerator`.
    pub struct SinkCompat<Si, Item> {
        #[pin]
        sink: Si,
        buf: Option<Item>,
    }
}

impl<Si, Item> SinkCompat<Si, Item> {
    /// Creates a new [`SinkCompat`], wrapping the given `Sink`.
    pub fn new(sink: Si) -> Self
    where
        Self: Sinkerator<Item>,
    {
        Self { sink, buf: None }
    }
}

impl<Si, Item> Sinkerator<Item> for SinkCompat<Si, Item>
where
    Si: futures::Sink<Item>,
{
    type Error = Si::Error;

    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        item: Option<Item>,
    ) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();
        assert!(
            item.is_none() || this.buf.is_none(),
            "Sinkerator not ready: `poll_send` must return `Ready` before another item may be sent."
        );

        let item = item.or_else(|| this.buf.take());
        if let Some(item) = item {
            match this.sink.as_mut().poll_ready(cx)? {
                Poll::Ready(()) => {
                    this.sink.start_send(item)?;
                }
                Poll::Pending => {
                    *this.buf = Some(item);
                    return Poll::Pending;
                }
            }
        }
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sink.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sink.poll_close(cx)
    }
}
