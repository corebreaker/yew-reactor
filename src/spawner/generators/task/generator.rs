use crate::spawner::{SpawnGenerator, FutureVoid};
use futures::executor::block_on;
use std::{fmt::{Debug, Formatter, Result as FmtResult}, thread::spawn};

pub struct TaskSpawner {

}

impl Default for TaskSpawner {
    fn default() -> Self {
        TaskSpawner {

        }
    }
}

impl Clone for TaskSpawner {
    fn clone(&self) -> Self {
        TaskSpawner {

        }
    }
}

impl Debug for TaskSpawner {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "TaskSpawner")
    }
}

impl SpawnGenerator for TaskSpawner {
    fn spawn(&self, fut: FutureVoid) {
        spawn(move || {
            block_on(fut);
        });
    }
}
