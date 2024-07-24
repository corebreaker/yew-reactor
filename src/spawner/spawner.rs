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
