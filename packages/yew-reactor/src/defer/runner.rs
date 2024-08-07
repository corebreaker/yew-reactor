use std::{sync::Arc, rc::Rc};

pub trait DeferRunner {
    fn run(&self, f: Arc<dyn Fn()>);
}

impl DeferRunner for Box<dyn DeferRunner> {
    fn run(&self, f: Arc<dyn Fn()>) {
        self.as_ref().run(f);
    }
}

impl DeferRunner for Rc<dyn DeferRunner> {
    fn run(&self, f: Arc<dyn Fn()>) {
        self.as_ref().run(f);
    }
}

impl DeferRunner for Arc<dyn DeferRunner> {
    fn run(&self, f: Arc<dyn Fn()>) {
        self.as_ref().run(f);
    }
}
