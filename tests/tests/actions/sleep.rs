use std::{time::{Duration, Instant}, future::Future};

pub(super) struct Sleep {
    instant: Instant
}

impl Sleep {
    pub(super) fn new() -> Self {
        Self {
            instant: Instant::now()
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if Instant::now() - self.instant >= Duration::from_secs_f64(1e-3) {
            std::task::Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    }
}
