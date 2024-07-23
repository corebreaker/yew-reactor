mod id;
mod signal;
mod runtime;
mod keyed_collection;

pub use self::{
    signal::Signal,
    runtime::Runtime,
    keyed_collection::KeyedCollection,
};

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{spawner::generators::FuturesSpawner, defer::runners::RunnerForTests};
    use std::sync::Arc;

    pub fn create_runtime() -> Arc<Runtime> {
        Runtime::new()
            .with_spawn_generator(FuturesSpawner::new())
            .with_defer_runner(RunnerForTests::new())
    }
}