use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;

pub mod sink;
pub mod workspace;

/// A stream that flattens collections of received events. If we have a stream that produces a
/// collection, wrap it in this stream to produce a stream that yields a single event at a time.
#[pin_project::pin_project]
struct FlatteningStream<I, S> {
    pending: VecDeque<I>,
    #[pin]
    inner: S,
}
impl<I, S> Stream for FlatteningStream<I, S>
where
    S: Stream<Item = VecDeque<I>>,
    I: std::fmt::Debug,
{
    type Item = I;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if let Some(event) = this.pending.pop_front() {
            return Poll::Ready(Some(event));
        }

        let Poll::Ready(events) = this.inner.poll_next(context) else {
            return Poll::Pending;
        };

        let Some(mut events) = events else {
            return Poll::Ready(None);
        };

        this.pending.append(&mut events);
        Poll::Ready(this.pending.pop_front())
    }
}

/// Convenience function for wrapping a stream in a [FlatteningStream].
fn flatten<I, S>(inner: S) -> impl Stream<Item = I>
where
    S: Stream<Item = VecDeque<I>>,
    I: std::fmt::Debug,
{
    FlatteningStream {
        pending: VecDeque::new(),
        inner,
    }
}
