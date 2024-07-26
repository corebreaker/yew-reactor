use std::sync::Arc;

pub trait DeferRunner {
    fn run(&self, f: Arc<dyn Fn()>);
}
