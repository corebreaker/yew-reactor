use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex, MutexGuard},
    task::Context,
    time::Duration,
};

/// A future that can reschedule itself to be polled by an `Executor`.
pub(super) struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

impl Task {
    /// Create a new task by sending it to the task queue.
    pub(super) fn new(future: BoxFuture<'static, ()>, task_sender: SyncSender<Arc<Task>>) -> Self {
        Task {
            future: Mutex::new(Some(future)),
            task_sender,
        }
    }

    pub(super) fn lock(&self) -> MutexGuard<Option<BoxFuture<'static, ()>>> {
        self.future.lock().unwrap()
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}
