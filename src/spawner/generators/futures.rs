use crate::spawner::{SpawnGenerator, FutureVoid};
use futures::executor::block_on;

#[derive(Clone, Debug, Default)]
pub struct FuturesSpawner;

impl FuturesSpawner {
    pub fn new() -> Self {
        FuturesSpawner
    }
}

impl SpawnGenerator for FuturesSpawner {
    fn spawn(&self, fut: FutureVoid) {
        block_on(fut);
    }
}
