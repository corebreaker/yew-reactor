use crate::defer::DeferRunner;
use std::sync::Arc;

pub(crate) struct RunnerForTests {}

impl RunnerForTests {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl DeferRunner for RunnerForTests {
    fn run(&self, f: Arc<dyn Fn()>) {
        f();
    }
}
