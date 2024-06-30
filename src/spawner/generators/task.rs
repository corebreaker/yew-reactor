use futures::FutureExt;
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
