use crate::backtrace::get_backtrace;
use futures::FutureExt;
use std::{
    task::{Context, Poll},
    pin::Pin,
    future::Future,
    panic::UnwindSafe,
    any::Any,
};
use std::fmt::Display;

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
                                None => String::from("Panicking for any reason with another type"),
                            },
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

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        panic::panic_any,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    #[tokio::test]
    async fn test_local_future() {
        let value = Arc::new(AtomicUsize::new(0));

        {
            let value = Arc::clone(&value);

            LocalFuture::new(async move {
                value.fetch_add(1, Ordering::Relaxed);
            })
            .await;
        }

        assert_eq!(value.load(Ordering::Relaxed), 1, "local future should be executed");
    }

    fn check_panic_result<R>(err: Result<R, Box<dyn Any + Send>>, msg: &str) {
        match err {
            Err(err) => match err.downcast_ref::<String>() {
                Some(err) => {
                    assert_eq!(err.lines().next(), Some(msg), "panic message should be `{msg}`",);
                }
                _ => {
                    panic!("panic should be a string");
                }
            },
            Ok(_) => {
                panic!("local future should panic")
            }
        }
    }

    #[tokio::test]
    async fn test_local_future_panic() {
        let value = Arc::new(AtomicUsize::new(0));

        {
            let value = Arc::clone(&value);
            let result = LocalFuture::new(async move {
                value.fetch_add(1, Ordering::Relaxed);
                panic_any(1234usize);
            })
            .catch_unwind()
            .await;

            check_panic_result(result, "Panicking for any reason with another type");
        }

        {
            let value = Arc::clone(&value);
            let result = LocalFuture::new(async move {
                value.fetch_add(1, Ordering::Relaxed);
                panic!("Panic with &str");
            })
            .catch_unwind()
            .await;

            check_panic_result(result, "Panic: Panic with &str");
        }

        {
            let value = Arc::clone(&value);
            let result = LocalFuture::new(async move {
                value.fetch_add(1, Ordering::Relaxed);
                panic_any(String::from("Panic with String"));
            })
            .catch_unwind()
            .await;

            check_panic_result(result, "Panic: Panic with String");
        }

        assert_eq!(
            value.load(Ordering::Relaxed),
            3,
            "local future should be executed 3 times"
        );
    }
}
// no-coverage:stop
