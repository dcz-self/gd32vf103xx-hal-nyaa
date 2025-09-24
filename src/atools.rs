/*! Async helpers */

use core::{future::poll_fn, task::Poll};

/// Busy-loops until f returns true.
pub async fn poll_until(mut f: impl FnMut() -> bool) {
    poll_fn(|cx| if f() {
            Poll::Ready(())
        } else {
            // TODO: this is maybe wrong.
            // Waker should maybe wake after returning
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    ).await
}
