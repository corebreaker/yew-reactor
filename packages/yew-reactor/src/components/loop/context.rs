use super::{super::ReactorContext, element::LoopElement};
use crate::signal::{Signal, Runtime};
use yew::{
    html::{Scope, AnyScope},
    Component,
    Context,
};

use std::sync::Arc;

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct LoopVar<T: Clone + Default + 'static> {
    rt:    Arc<Runtime>,
    value: Option<Signal<Option<T>>>,
}

impl<T: Clone + Default + 'static> LoopVar<T> {
    pub(in super::super) fn new(value: Option<Signal<Option<T>>>, rt: Option<Arc<Runtime>>) -> Self {
        let rt = rt
            .or_else(|| value.as_ref().map(|v| v.runtime()))
            .unwrap_or_else(Runtime::new);

        Self {
            rt,
            value,
        }
    }

    pub fn runtime(&self) -> Arc<Runtime> {
        Arc::clone(&self.rt)
    }

    pub fn with_value<R, F>(&self, f: F) -> R
    where
        for<'a> F: FnOnce(Option<&'a T>) -> R, {
        match self.value.as_ref() {
            Some(value) => value.with(|v| f(v.as_ref())),
            None => f(None),
        }
    }

    pub fn get_value(&self) -> Option<T> {
        self.value.as_ref().and_then(|v| v.with(|v| v.clone()))
    }
}

#[derive(Clone)]
pub(crate) struct LoopDataContext<T: 'static> {
    signal: Signal<Option<T>>,
}

impl<T: Clone + Default + PartialEq + 'static> LoopDataContext<T> {
    pub(super) fn new(signal: Signal<Option<T>>) -> Self {
        Self {
            signal,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_var(&self) -> LoopVar<T> {
        LoopVar::new(Some(self.signal.clone()), Some(self.signal.runtime()))
    }
}

impl<T: 'static> Eq for LoopDataContext<T> {}

impl<T: 'static> PartialEq for LoopDataContext<T> {
    fn eq(&self, other: &Self) -> bool {
        self.signal == other.signal
    }
}

/// Component extension trait for Loop component
pub trait LoopContext {
    fn get_loop_var<T: Clone + Default + PartialEq + 'static>(&self) -> LoopVar<T>;
}

impl<C: Component> LoopContext for Context<C> {
    fn get_loop_var<T: Clone + Default + PartialEq + 'static>(&self) -> LoopVar<T> {
        self.link().get_loop_var()
    }
}

impl<C: Component> LoopContext for Scope<C> {
    fn get_loop_var<T: Clone + Default + PartialEq + 'static>(&self) -> LoopVar<T> {
        LoopVar::new(get_loop_var_from_scope(self), self.runtime())
    }
}

fn get_loop_var_from_scope<C, T>(scope: &Scope<C>) -> Option<Signal<Option<T>>>
where
    C: Component,
    T: Clone + Default + PartialEq + 'static, {
    get_loop_var(scope.get_parent()?)
}

fn get_loop_var<T: Clone + Default + PartialEq + 'static>(scope: &AnyScope) -> Option<Signal<Option<T>>> {
    match scope.try_downcast::<LoopElement<T>>() {
        None => get_loop_var(scope.get_parent()?),
        Some(scope) => Some(scope.get_component()?.get_signal()),
    }
}
