use super::FutureVoid;
use std::{sync::Arc, rc::Rc};

pub trait SpawnGenerator {
    fn spawn(&self, fut: FutureVoid);
}

impl SpawnGenerator for Box<dyn SpawnGenerator> {
    fn spawn(&self, fut: FutureVoid) {
        self.as_ref().spawn(fut);
    }
}

impl SpawnGenerator for Rc<dyn SpawnGenerator> {
    fn spawn(&self, fut: FutureVoid) {
        self.as_ref().spawn(fut);
    }
}

impl SpawnGenerator for Arc<dyn SpawnGenerator> {
    fn spawn(&self, fut: FutureVoid) {
        self.as_ref().spawn(fut);
    }
}
