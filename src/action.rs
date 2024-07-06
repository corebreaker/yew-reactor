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
    pending: Signal<bool>,
    lock: Arc<Mutex<()>>,
    value: Signal<Option<O>>,
    action_fn: Rc<dyn Fn(I) -> LocalFuture<O>>,
}

impl<I, O: UnwindSafe + 'static> Action<I, O> {
    pub(crate) fn new<R, F>(runtime: Arc<Runtime>, action_fn: F) -> Self
        where R: Future<Output = O> + UnwindSafe + 'static, F: Fn(I) -> R + 'static {
        let id = new_id();
        let pending = Arc::clone(&runtime).create_signal(false);
        let value = Arc::clone(&runtime).create_signal(None::<O>);
        let lock = Arc::new(Mutex::new(()));
        let action_fn = Rc::new(move |input: I| LocalFuture::new(action_fn(input)));

        Self {
            id,
            runtime,
            pending,
            lock,
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
        let lock = Arc::clone(&self.lock);

        self.pending.with(|p| {
            let _lock = lock.lock().unwrap();

            if *p {
                true
            } else {
                false
            }
        })
    }

    pub fn dispatch(&self, input: I) {
        let lock = Arc::clone(&self.lock);

        {
            let _lock = lock.lock().unwrap();
            if self.pending.peek() {
                return;
            }
        }

        let fut = Rc::clone(&self.action_fn)(input);
        let output = self.value.clone();
        let pending = self.pending.clone();
        let set_pending = move |val| {
            let _lock = lock.lock().unwrap();

            pending.set(val);
        };

        self.runtime().spawn(async move {
            output.untracked_update(|v| { v.take(); });
            set_pending(true);

            let res = fut.await;

            set_pending(false);
            output.update(|o| { o.replace(res); });
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
