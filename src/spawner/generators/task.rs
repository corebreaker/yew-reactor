use crate::spawner::{SpawnGenerator, FutureVoid};
use tokio::spawn;

#[derive(Clone, Debug, Default)]
pub struct TaskSpawner;

impl TaskSpawner {
    pub fn new() -> Self {
        TaskSpawner
    }
}

impl SpawnGenerator for TaskSpawner {
    fn spawn(&self, fut: FutureVoid) {
        spawn(fut);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{atomic::{AtomicUsize, AtomicBool, Ordering}, Arc};
    use tokio::task::yield_now;

    #[tokio::test]
    async fn test_spawn() {
        let lock = Arc::new(AtomicBool::new(false));
        let value = Arc::new(AtomicUsize::new(0));

        {
            let lock = Arc::clone(&lock);
            let value = Arc::clone(&value);

            TaskSpawner::new().spawn(FutureVoid::new(async move {
                value.fetch_add(1, Ordering::Relaxed);
                lock.store(true, Ordering::Relaxed);
            }));
        }

        while !lock.load(Ordering::Relaxed) {
            yield_now().await;
        }

        assert_eq!(value.load(Ordering::Relaxed), 1, "spawned task should be executed");
    }
}
