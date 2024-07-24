use super::{generators::default::DefaultSpawner, SpawnGenerator, FutureVoid};
use std::{future::Future, sync::RwLock, panic::UnwindSafe, sync::Arc};

#[derive(Default)]
pub struct Spawner(RwLock<Option<Arc<dyn SpawnGenerator>>>);

impl Spawner {
    pub fn reset_generator(&self) {
        self.0.write().unwrap().take();
    }

    pub fn set_generator(&self, generator: impl SpawnGenerator + 'static) {
        self.0.write().unwrap().replace(Arc::new(generator));
    }

    pub fn spawn<F: Future<Output = ()> + UnwindSafe + 'static>(&self, f: F) {
        self.init_spawner();

        let generator = self.0.read().unwrap().as_ref().map(|r| Arc::clone(r));
        if let Some(generator) = generator {
            generator.spawn(FutureVoid::new(f));
        }
    }

    fn init_spawner(&self) {
        let mut this = self.0.write().unwrap();

        if let None = this.as_ref() {
            this.replace(Arc::new(DefaultSpawner));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::generators::FuturesSpawner;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_init_spawner() {
        let spawner = Spawner::default();

        assert!(spawner.0.read().unwrap().is_none(), "the spawner should be None before init");
        spawner.init_spawner();
        assert!(spawner.0.read().unwrap().is_some(), "the spawner should be Some after init");
    }

    #[test]
    fn test_reset_generator() {
        let spawner = Spawner::default();
        spawner.init_spawner();

        assert!(spawner.0.read().unwrap().is_some(), "the spawner should be Some before reset");
        spawner.reset_generator();
        assert!(spawner.0.read().unwrap().is_none(), "the spawner should be None after reset");
    }

    #[test]
    fn test_set_generator() {
        let spawner = Spawner::default();

        assert!(spawner.0.read().unwrap().is_none(), "the spawner should be None before init");
        spawner.set_generator(DefaultSpawner);
        assert!(spawner.0.read().unwrap().is_some(), "the spawner should be Some after set");
    }

    #[test]
    fn test_spawn() {
        let spawner = Spawner::default();
        spawner.set_generator(FuturesSpawner::new());

        let value = Arc::new(AtomicUsize::new(0));

        {
            let value = Arc::clone(&value);

            spawner.spawn(async move {
                value.fetch_add(1, Ordering::Relaxed);
            });
        }

        assert_eq!(value.load(Ordering::Relaxed), 1, "spawned future should be executed");
    }
}
