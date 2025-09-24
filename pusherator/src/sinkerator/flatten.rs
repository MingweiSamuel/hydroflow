use std::pin::Pin;
use std::task::{ready, Context, Poll};

use pin_project_lite::pin_project;

use super::Sinkerator;

pin_project! {
    /// An [`Sinkerator`] which map each items using `Func` before sending them to the sink.
    pub struct Flatten<Si, Iter> {
        #[pin]
        si: Si,
        iter: Option<Iter>,
    }
}

impl<Si, Iter> Flatten<Si, Iter> {
    /// Creates a new [`Flatten`], which will flatten items using [`IntoIterator::into_iter`] before sending the outputs to `si`.
    pub fn new<Item>(si: Si) -> Self
    where
        Self: Sinkerator<Item>
    {
        Self { si, iter: None }
    }
}

impl<Si, Item> Sinkerator<Item> for Flatten<Si, Item::IntoIter>
where
    Item: IntoIterator,
    Si: Sinkerator<Item::Item>,
{
    type Error = Si::Error;

    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        item: Option<Item>,
    ) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        if let Some(item) = item {
            debug_assert!(this.iter.is_none(), "Sinkerator not ready: `poll_send` must return `Ready` before another item may be sent.");
            *this.iter = Some(item.into_iter());
        } else {
            ready!(this.si.as_mut().poll_send(cx, None)?);
        }

        if let Some(iter) = this.iter.as_mut() {
            while let Some(item) = iter.next() {
                ready!(this.si.as_mut().poll_send(cx, Some(item))?);
            }
        }

        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().si.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().si.poll_close(cx)
    }
}
