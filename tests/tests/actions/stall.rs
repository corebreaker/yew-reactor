use super::sleep::Sleep;
use std::{sync::atomic::{AtomicBool, Ordering}, time::Duration};

#[derive(Debug)]
pub(super) struct Stall {
    lock: AtomicBool,
}

impl Stall {
    pub(super) fn new() -> Self {
        Self {
            lock: AtomicBool::new(true),
        }
    }

    pub(super) async fn wait(&self) {
        while self.lock.load(Ordering::Relaxed) {
            Sleep::new().await;
        }
    }

    pub fn notify(&self) {
        self.lock.store(false, Ordering::Relaxed);
    }
}
