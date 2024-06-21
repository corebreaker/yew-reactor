use std::{task::{Context, Poll}, pin::Pin, future::Future};

pub type FutureVoid = LocalFuture<()>;

pub struct LocalFuture<O> {
    future: Pin<Box<dyn Future<Output = O>>>,
}

impl<O> LocalFuture<O> {
    pub fn new<F: Future<Output = O> + 'static>(f: F) -> Self {
        Self {
            future: Box::pin(f),
        }
    }
}

impl<O> Future for LocalFuture<O> {
    type Output = O;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.as_mut().poll(cx)
    }
}

unsafe impl<O> Send for LocalFuture<O> {}
unsafe impl<O> Sync for LocalFuture<O> {}
