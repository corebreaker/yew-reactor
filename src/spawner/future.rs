use crate::backtrace::get_backtrace;
use futures::FutureExt;
use std::{task::{Context, Poll}, pin::Pin, future::Future, panic::UnwindSafe, any::Any};

pub type FutureVoid = LocalFuture<()>;

pub struct LocalFuture<O> {
    future: Pin<Box<dyn Future<Output = O>>>,
}

impl<O> LocalFuture<O> {
    pub fn new<F: Future<Output = O> + UnwindSafe + 'static>(f: F) -> Self {
        Self {
            future: Box::pin(async {
                match f.catch_unwind().await {
                    Ok(v) => v,
                    Err(err) => {
                        let message = match err.downcast_ref::<&str>() {
                            Some(err) => format!("Panic: {err}"),
                            None => match err.downcast_ref::<String>() {
                                Some(err) => format!("Panic: {err}"),
                                None => format!("Panicking for any reason with type {:?}", err.type_id()),
                            }
                        };

                        let backtrace = get_backtrace(2);

                        panic!("{message}\n{backtrace:?}");
                    }
                }
            }),
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

impl<O> UnwindSafe for LocalFuture<O> {}