use crate::spawner::{SpawnGenerator, FutureVoid};
use futures::executor::block_on;

#[derive(Clone, Debug, Default)]
pub struct FuturesSpawner {}

impl FuturesSpawner {
    pub fn new() -> Self {
        FuturesSpawner{}
    }
}

impl SpawnGenerator for FuturesSpawner {
    fn spawn(&self, fut: FutureVoid) {
        block_on(fut);
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

    #[tokio::test]
    async fn test_spawn() {
        let value = Arc::new(AtomicUsize::new(0));

        {
            let value = Arc::clone(&value);

            FuturesSpawner::new().spawn(FutureVoid::new(async move {
                value.fetch_add(1, Ordering::Relaxed);
            }));
        }

        assert_eq!(value.load(Ordering::Relaxed), 1, "spawned future should be executed");
    }
}
// no-coverage:stop
