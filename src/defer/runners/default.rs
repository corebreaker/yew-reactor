use crate::defer::DeferRunner;
use yew::platform::spawn_local;
use std::sync::Arc;

pub(in super::super) struct DefaultRunner {}

impl DefaultRunner {
    pub(in super::super) fn new() -> Self {
        Self {}
    }
}

impl DeferRunner for DefaultRunner {
    fn run(&self, f: Arc<dyn Fn()>) {
        spawn_local(async move {
            let f = Arc::clone(&f);

            f();
        });
    }
}