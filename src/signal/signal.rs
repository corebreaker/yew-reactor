use super::{id::SignalId, Runtime};
use std::{fmt::{Display, Debug, Formatter, Result as FmtResult}, marker::PhantomData, sync::Arc};

pub struct Signal<T: 'static> {
    runtime: Arc<Runtime>,
    id: SignalId,
    ty: PhantomData<T>,
}

impl<T: 'static> Signal<T> {
    pub(super) fn new(runtime: Arc<Runtime>, id: SignalId) -> Self {
        runtime.inc_signal_ref(id);

        Self {
            runtime,
            id,
            ty: PhantomData,
        }
    }

    pub(super) fn id(&self) -> SignalId {
        self.id
    }

    pub fn runtime(&self) -> Arc<Runtime> {
        Arc::clone(&self.runtime)
    }

    pub fn set(&self, value: T) {
        self.update(|v| *v = value);
    }

    pub fn untracked_set(&self, value: T) {
        self.untracked_update(|v| *v = value);
    }

    pub fn with<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        let runtime = self.runtime();

        // add subscribers
        runtime.add_subscriber(self.id);

        // get value
        let signal_id = runtime.fetch_linked_signal_id(&self.id);
        let values = runtime.get_signal_values().borrow();
        let value = values[&signal_id].borrow();
        let signal_value = value.downcast_ref::<T>().unwrap();

        // return value
        f(signal_value)
    }

    pub fn with_another<X: 'static, R, F: FnOnce(&T, &X) -> R>(&self, other: Signal<X>, f: F) -> R {
        let other = other.clone();

        self.with(move |v| other.with(|o| f(v, o)))
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        let runtime = self.runtime();
        let id = self.id;

        // set value
        {
            let id = runtime.fetch_linked_signal_id(&id);
            let values = runtime.get_signal_values().borrow();
            let mut value = values[&id].borrow_mut();
            let signal_value = value.downcast_mut::<T>().unwrap();

            f(signal_value);
        }

        // notify subscribers
        self.runtime().spawn(async move {
            runtime.notify_subscribers(id);
        });
    }

    pub fn untracked_update(&self, f: impl FnOnce(&mut T)) {
        let runtime = self.runtime();
        let values = runtime.get_signal_values().borrow();
        let mut value = values[&self.id].borrow_mut();
        let signal_value = value.downcast_mut::<T>().unwrap();

        // set value
        f(signal_value);
    }

    pub fn update_if(&self, f: impl FnOnce(&mut T) -> bool) {
        let runtime = self.runtime();
        let id = self.id;

        // set value
        let should_notify = {
            let values = runtime.get_signal_values().borrow();
            let mut value = values[&id].borrow_mut();
            let signal_value = value.downcast_mut::<T>().unwrap();

            f(signal_value)
        };

        // notify subscribers
        if should_notify {
            self.runtime().spawn(async move {
                runtime.notify_subscribers(id);
            });
        }
    }

    pub fn link_to(&self, source: &Signal<T>) {
        self.runtime.link_signal(self.id, source.id);
    }

    pub fn create_link(&self) -> Signal<T> {
        Arc::clone(&self.runtime).create_link(self.id)
    }
}

impl<T> Display for Signal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Signal[{}]", self.id.id())
    }
}

impl<T: Debug> Debug for Signal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let id = self.id.id();

        self.with(|v| write!(f, "Signal[{id} = {v:?}]"))
    }
}

impl<T: Clone + 'static> Signal<T> {
    pub(crate) fn peek(&self) -> T {
        let runtime = self.runtime();

        // get value
        let signal_id = runtime.fetch_linked_signal_id(&self.id);
        let values = runtime.get_signal_values().borrow();
        let value = values[&signal_id].borrow();

        // return value
        value.downcast_ref::<T>().unwrap().clone()
    }
}

impl<T: Clone + 'static> Signal<T> {
    pub fn get(&self) -> T {
        self.with(T::clone)
    }
}

impl<T: 'static> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: 'static> Eq for Signal<T> {}

impl<T: 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self::new(self.runtime(), self.id)
    }
}

impl<T: 'static> Drop for Signal<T> {
    fn drop(&mut self) {
        self.runtime.clean_signal(self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spawner::generators::FuturesSpawner;

    #[test]
    fn test_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(0);

        assert_eq!(signal.get(), 0, "signal value should be equal to the initial value");

        signal.set(1);
        assert_eq!(signal.get(), 1, "signal value should be equal to the set value");

        signal.update(|v| *v = 2);
        assert_eq!(signal.get(), 2, "signal value should be equal to the updated value");

        signal.untracked_update(|v| *v = 3);
        assert_eq!(signal.get(), 3, "signal value should be equal to the untracked updated value");

        signal.update_if(|v| {
            *v = 4;
            true
        });
        assert_eq!(signal.get(), 4, "signal value should be equal to the updated value");

        signal.update_if(|v| {
            *v = 5;
            false
        });
        assert_eq!(signal.get(), 5, "signal value should be equal to the previous value");

        let signal2 = Arc::clone(&rt).create_signal(0);
        signal.link_to(&signal2);
        signal2.set(6);
        assert_eq!(signal2.get(), 6, "signal value should be equal to the linked signal value with linked getter");
        assert_eq!(signal.get(), 6, "signal value should be equal to the linked signal value with direct getter");

        let signal3 = signal.create_link();
        signal3.set(7);

        assert_eq!(
            signal3.get(),
            7,
            "signal value should be equal to the created linked signal value with linked getter",
        );

        assert_eq!(
            signal.get(),
            7,
            "signal value should be equal to the created linked signal value with direct getter",
        );
    }
}
