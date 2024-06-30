use super::{generators::default::DefaultSpawner, SpawnGenerator, FutureVoid};
use std::{future::Future, cell::RefCell, panic::UnwindSafe};

#[derive(Default)]
pub struct Spawner(RefCell<Option<Box<dyn SpawnGenerator>>>);

impl Spawner {
    pub fn new() -> Self {
        Spawner(RefCell::new(None))
    }

    pub fn reset_generator(&self) {
        self.0.borrow_mut().take();
    }

    pub fn set_generator(&self, generator: impl SpawnGenerator + 'static) {
        self.0.borrow_mut().replace(Box::new(generator));
    }

    pub fn spawn<F: Future<Output = ()> + UnwindSafe + 'static>(&self, f: F) {
        if self.has_no_generator() {
            self.set_generator(DefaultSpawner);
        }

        if let Some(generator) = self.0.borrow().as_ref() {
            generator.spawn(FutureVoid::new(f));
        }
    }

    fn has_no_generator(&self) -> bool {
        self.0.borrow().is_none()
    }
}
