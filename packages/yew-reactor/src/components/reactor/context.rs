use super::Reactor;
use crate::signal::Runtime;
use yew::{
    html::{Scope, AnyScope},
    Component,
    Context,
};
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub(in super::super) struct ReactorDataContext {
    rt: Arc<Runtime>,
}

impl ReactorDataContext {
    pub(super) fn new(rt: Arc<Runtime>) -> Self {
        Self {
            rt,
        }
    }

    pub(in super::super) fn runtime(&self) -> Arc<Runtime> {
        Arc::clone(&self.rt)
    }
}

/// Component extension trait for Reactor component
pub trait ReactorContext {
    fn runtime(&self) -> Option<Arc<Runtime>>;
}

impl<C: Component> ReactorContext for Context<C> {
    fn runtime(&self) -> Option<Arc<Runtime>> {
        self.link().runtime()
    }
}

impl<C: Component> ReactorContext for Scope<C> {
    fn runtime(&self) -> Option<Arc<Runtime>> {
        get_runtime(self.get_parent()?)
    }
}

fn get_runtime(scope: &AnyScope) -> Option<Arc<Runtime>> {
    match scope.try_downcast::<Reactor>() {
        None => get_runtime(scope.get_parent()?),
        Some(scope) => Some(scope.get_component()?.runtime()),
    }
}
