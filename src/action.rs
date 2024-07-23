use super::signal::{Runtime, Signal};
use crate::{spawner::LocalFuture, id_generator::new_id};
use uuid::Uuid;
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    sync::{Mutex, Arc},
    panic::UnwindSafe,
    future::Future,
    rc::Rc,
};

#[derive(Clone)]
pub struct Action<I, O: UnwindSafe + 'static> {
    id: Uuid,
    runtime: Arc<Runtime>,
    pending: Arc<Mutex<bool>>,
    value: Signal<Option<O>>,
    action_fn: Rc<dyn Fn(I) -> LocalFuture<O>>,
}

impl<I, O: UnwindSafe + 'static> Action<I, O> {
    pub(crate) fn new<R, F>(runtime: Arc<Runtime>, action_fn: F) -> Self
        where R: Future<Output = O> + UnwindSafe + 'static, F: Fn(I) -> R + 'static {
        let id = new_id();
        let pending = Arc::new(Mutex::new(false));
        let value = Arc::clone(&runtime).create_signal(None::<O>);
        let action_fn = Rc::new(move |input: I| LocalFuture::new(action_fn(input)));

        Self {
            id,
            runtime,
            pending,
            value,
            action_fn,
        }
    }

    pub fn runtime(&self) -> Arc<Runtime> {
        Arc::clone(&self.runtime)
    }

    pub fn value(&self) -> Signal<Option<O>> {
        self.value.clone()
    }

    pub fn with_value<R, F: FnOnce(&Option<O>) -> R>(&self, f: F) -> R {
        self.value.with(f)
    }

    pub fn is_pending(&self) -> bool {
        *self.pending.lock().unwrap()
    }

    pub fn dispatch(&self, input: I) {
        {
            let pending_lock = Arc::clone(&self.pending);
            let mut pending_check = pending_lock.lock().unwrap();
            if *pending_check {
                return;
            }

            *pending_check = true;
        }

        let fut = Rc::clone(&self.action_fn)(input);
        let output = self.value.clone();
        let pending = Arc::clone(&self.pending);

        self.runtime().spawn(async move {
            output.untracked_update(|v| { v.take(); });

            let res = fut.await;

            output.update(|o| { o.replace(res); });

            {
                let pending_lock = Arc::clone(&pending);
                let mut pending = pending_lock.lock().unwrap();

                *pending = false;
            }
        });
    }
}

impl<I, O: Clone + UnwindSafe + 'static> Action<I, O> {
    pub fn get(&self) -> Option<O> {
        self.value.get()
    }
}

impl<I, O: UnwindSafe> Debug for Action<I, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Action[{}; pending: {}]", self.id, self.is_pending())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::tests::create_runtime;
    use std::sync::RwLock;

    fn create_action() -> (Action<i32, i32>, Arc<RwLock<i32>>) {
        let action = create_runtime().create_action(|i| async move { i * 2 });
        let value = Arc::new(RwLock::new(123));

        {
            let value = Arc::clone(&value);
            let output = action.value().clone();

            action.runtime().create_effect(move || {
                if let Some(v) = output.with(|v: &Option<i32>| -> Option<i32> { v.as_ref().map(|v| *v + 2) }) {
                    let mut value = value.write().unwrap();

                    *value = v;
                }
            });
        }

        (action, value)
    }

    #[test]
    fn test_create_action() {
        let (action, _) = create_action();

        assert_eq!(action.is_pending(), false, "action should not be pending");
        assert_eq!(action.get(), None, "action value should be None");

        assert_eq!(
            format!("{:?}", action),
            format!("Action[{}; pending: false]", action.id),
            "action should be formatted correctly",
        );
    }

    #[test]
    fn test_changes_on_value() {
        let (action, value) = create_action();

        action.value().set(Some(40));
        assert_eq!(*value.read().unwrap(), 42, "action value should be set");
    }

    #[test]
    fn test_dispatch_action() {
        let (action, value) = create_action();

        action.dispatch(20);
        assert_eq!(*value.read().unwrap(), 42, "action value should be set");
    }

    #[test]
    fn test_dispatch_while_pending_previews_dispatch() {
        let (action, value) = create_action();

        {
            let mut pending = action.pending.lock().unwrap();

            *pending = true;
        }

        action.dispatch(20);
        assert_eq!(*value.read().unwrap(), 123, "action value should be set");
    }
}
