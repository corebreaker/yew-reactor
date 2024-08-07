use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    sync::Arc,
    any::Any,
};

#[derive(Clone)]
pub(super) struct Function(Arc<dyn Fn(Option<&String>) -> String>);

impl Function {
    pub(super) fn new(f: impl Fn(Option<&String>) -> String + 'static) -> Self {
        Self(Arc::new(f))
    }

    pub(super) fn get(&self) -> Arc<dyn Fn(Option<&String>) -> String> {
        Arc::clone(&self.0)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Function[{:?}]", self.0.as_ref().type_id())
    }
}
