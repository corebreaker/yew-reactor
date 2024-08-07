use std::sync::Arc;

#[derive(Clone)]
pub(super) struct Function(Arc<dyn Fn()>);

impl Function {
    pub(super) fn new(f: Arc<dyn Fn()>) -> Self {
        Self(f)
    }

    pub(super) fn get(&self) -> Arc<dyn Fn()> {
        Arc::clone(&self.0)
    }
}

unsafe impl Send for Function {}
