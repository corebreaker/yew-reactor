use super::{generators::default::DefaultSpawner, SpawnGenerator, FutureVoid};
use std::{future::Future, sync::RwLock, panic::UnwindSafe};

#[derive(Default)]
pub struct Spawner(RwLock<Option<Box<dyn SpawnGenerator>>>);

impl Spawner {
    pub fn reset_generator(&self) {
        self.0.write().unwrap().take();
    }

    pub fn set_generator(&self, generator: impl SpawnGenerator + 'static) {
        self.0.write().unwrap().replace(Box::new(generator));
    }

    pub fn spawn<F: Future<Output = ()> + UnwindSafe + 'static>(&self, f: F) {
        self.init();

        if let Some(generator) = self.0.read().unwrap().as_ref() {
            generator.spawn(FutureVoid::new(f));
        }
    }

    fn init(&self) {
        let mut this = self.0.write().unwrap();

        if let None = this.as_ref() {
            this.replace(Box::new(DefaultSpawner));
        }
    }
}
