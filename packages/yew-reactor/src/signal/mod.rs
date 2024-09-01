mod id;
mod keyed_collection;
mod mapping;
mod runtime;
mod signal;

pub use self::{signal::Signal, mapping::SignalMap, runtime::Runtime, keyed_collection::KeyedCollection};

// no-coverage:start
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
// no-coverage:stop
