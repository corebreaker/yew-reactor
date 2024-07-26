use yew_reactor::spawner::LocalFuture;
use std::{fmt::{Debug, Formatter, Result as FmtResult}, sync::Arc};

#[derive(Clone)]
pub(super) struct Function {
    name: String,
    func: Arc<dyn Fn() -> LocalFuture<&'static str> + 'static>,
}

impl Function {
    pub(super) fn new(name: &str, func: impl Fn() -> LocalFuture<&'static str> + 'static) -> Self {
        Self {
            name: name.to_string(),
            func: Arc::new(func),
        }
    }

    pub(super) fn call(&self) -> LocalFuture<&'static str> {
        (self.func)()
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Function[{}]", self.name)
    }
}
