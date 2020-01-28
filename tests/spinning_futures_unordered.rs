use futures::{
    future::poll_fn,
    stream::{FuturesUnordered, Stream as _},
};
use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use unicycle::Unordered;

struct Spinner<'a>(&'a Cell<usize>);

impl Future for Spinner<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.set(self.0.get() + 1);

        // Note: this will not be needed once we have a futures release with:
        // https://github.com/rust-lang/futures-rs/pull/2049
        if self.0.get() > 16 {
            return Poll::Ready(());
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

#[tokio::test]
async fn test_spinning_futures_unordered() {
    let count = Cell::new(0);

    let futures = FuturesUnordered::new();
    futures.push(Spinner(&count));
    pin_utils::pin_mut!(futures);

    let _ = poll_fn::<(), _>(move |cx| {
        let _ = Pin::new(&mut futures).poll_next(cx);
        Poll::Ready(())
    })
    .await;

    // Note: FuturesUnordered will spin a bit before yielding.
    assert!(count.get() > 1);
}

#[tokio::test]
async fn test_spinning_unordered() {
    let count = Cell::new(0);

    let mut futures = Unordered::new();
    futures.push(Spinner(&count));
    pin_utils::pin_mut!(futures);

    let _ = poll_fn::<(), _>(move |cx| {
        let _ = Pin::new(&mut futures).poll_next(cx);
        Poll::Ready(())
    })
    .await;

    // Note: Unicycle guarantees each future is poll at most once.
    assert_eq!(1, count.get());
}
