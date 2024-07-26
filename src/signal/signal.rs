use super::{id::SignalId, Runtime};
use std::{
    fmt::{Display, Debug, Formatter, Result as FmtResult},
    sync::atomic::{AtomicBool, Ordering},
    marker::PhantomData,
    sync::Arc,
};

pub struct Signal<T: 'static> {
    runtime: Arc<Runtime>,
    id: SignalId,
    registered: AtomicBool,
    ty: PhantomData<T>,
}

impl<T: 'static> Signal<T> {
    pub(super) fn new(runtime: Arc<Runtime>, id: SignalId) -> Self {
        runtime.inc_signal_ref(id);

        Self {
            runtime,
            id,
            registered: AtomicBool::new(false),
            ty: PhantomData,
        }
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
        if runtime.add_subscriber(self.id) {
            if !self.registered.fetch_or(true, Ordering::SeqCst) {
                runtime.dec_signal_ref(self.id);
            }
        }

        // get value
        let value_ref = {
            let signal_id = runtime.get_source_id(self.id);

            runtime.get_value(&signal_id)
        };

        let value = value_ref.read().unwrap();
        let signal_value = value.downcast_ref::<T>().unwrap();

        // return value
        f(signal_value)
    }

    pub fn with_another<X: 'static, R, F: FnOnce(&T, &X) -> R>(&self, other: Signal<X>, f: F) -> R {
        let other = other.clone();

        self.with(move |v| other.with(|o| f(v, o)))
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        let id = self.id;
        let runtime = self.runtime();

        // set value
        {
            let value_ref = {
                let signal_id = runtime.get_source_id(id);

                runtime.get_value(&signal_id)
            };

            let mut value = value_ref.write().unwrap();
            let signal_value = value.downcast_mut::<T>().unwrap();

            f(signal_value);
        }

        // notify subscribers
        self.runtime().defer(move || {
            runtime.notify_subscribers(id);
        });
    }

    pub fn untracked_update(&self, f: impl FnOnce(&mut T)) {
        let value_ref = {
            let runtime = self.runtime();
            let signal_id = runtime.get_source_id(self.id);

            runtime.get_value(&signal_id)
        };

        let mut value = value_ref.write().unwrap();
        let signal_value = value.downcast_mut::<T>().unwrap();

        f(signal_value);
    }

    pub fn update_if(&self, f: impl FnOnce(&mut T) -> bool) {
        let runtime = self.runtime();
        let id = self.id;

        // set value
        let should_notify = {
            let value_ref = {
                let signal_id = runtime.get_source_id(id);

                runtime.get_value(&signal_id)
            };

            let mut value = value_ref.write().unwrap();
            let signal_value = value.downcast_mut::<T>().unwrap();

            f(signal_value)
        };

        // notify subscribers
        if should_notify {
            self.runtime().defer(move || {
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
        let signal_id = runtime.get_source_id(self.id);
        let value_ref = runtime.get_value(&signal_id);
        let value = value_ref.read().unwrap();

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

unsafe impl<T: 'static> Send for Signal<T> {}

// no-coverage:start
#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use super::*;
    use crate::signal::tests::create_runtime;

    impl<T: 'static> Signal<T> {
        pub(in super::super) fn id(&self) -> SignalId {
            self.id
        }
    }

    #[test]
    fn test_signal_get() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(42);

        assert_eq!(signal.get(), 42, "signal value should be equal to the initial value");
    }

    #[test]
    fn test_signal_with() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(21);

        assert_eq!(signal.with(|v| *v * 2), 42, "signal value should be equal to the initial value");
    }

    #[test]
    fn test_signal_set() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(0);

        assert_eq!(signal.get(), 0, "signal value should be equal to the initial value");

        signal.set(1);
        assert_eq!(signal.get(), 1, "signal value should be equal to the set value");
    }

    #[test]
    fn test_signal_update() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(0);

        assert_eq!(signal.get(), 0, "signal value should be equal to the initial value");
        signal.update(|v| *v = 2);
        assert_eq!(signal.get(), 2, "signal value should be equal to the updated value");
    }

    #[test]
    fn test_signal_link() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(0);

        assert_eq!(signal.get(), 0, "signal value should be equal to the initial value");

        let signal2 = Arc::clone(&rt).create_signal(0);
        signal.link_to(&signal2);
        signal2.set(6);
        assert_eq!(signal2.get(), 6, "signal value should be equal to the linked signal value with linked getter");
        assert_eq!(signal.get(), 6, "signal value should be equal to the linked signal value with direct getter");

        let signal3 = signal.create_link();
        signal3.set(7);
    }

    #[test]
    fn test_combine_signal() {
        let rt = create_runtime();
        let signal1 = Arc::clone(&rt).create_signal(1);
        let signal2 = Arc::clone(&rt).create_signal(2);
        let result = signal1.with_another(signal2, |v1, v2| v1 + v2);

        assert_eq!(result, 3, "signal value should be equal to the sum of the two signals");
    }

    #[test]
    fn test_signal_update_in_an_effect() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(42);
        let value = Arc::new(AtomicUsize::new(0));
        let call_count = Arc::new(AtomicUsize::new(0));

        {
            let value = Arc::clone(&value);
            let call_count = Arc::clone(&call_count);
            let signal = signal.clone();

            signal.runtime().create_effect(move || {
                call_count.fetch_add(1, Ordering::SeqCst);
                value.store(signal.get(), Ordering::SeqCst);
            });
        }

        assert_eq!(call_count.load(Ordering::SeqCst), 1, "the effect should be called once for initial value");
        assert_eq!(value.load(Ordering::SeqCst), 42, "the initial value should be stored");

        signal.untracked_update(|v| *v = 123);
        assert_eq!(signal.get(), 123, "signal value should be equal to the untracked updated value");
        assert_eq!(call_count.load(Ordering::SeqCst), 1, "the effect should not be called for untracked update");
        assert_eq!(value.load(Ordering::SeqCst), 42, "the stored value should not be changed");

        signal.update(|v| *v = 0);
        assert_eq!(signal.get(), 0, "signal value should be equal to the untracked updated value");
        assert_eq!(call_count.load(Ordering::SeqCst), 2, "the effect should be called for untracked update");
        assert_eq!(value.load(Ordering::SeqCst), 0, "the stored value should be changed");
    }

    #[test]
    fn test_signal_conditionned_update() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(42);
        let value = Arc::new(AtomicUsize::new(0));
        let call_count = Arc::new(AtomicUsize::new(0));

        {
            let value = Arc::clone(&value);
            let call_count = Arc::clone(&call_count);
            let signal = signal.clone();

            signal.runtime().create_effect(move || {
                call_count.fetch_add(1, Ordering::SeqCst);
                value.store(signal.get(), Ordering::SeqCst);
            });
        }

        assert_eq!(call_count.load(Ordering::SeqCst), 1, "the effect should be called once for initial value");
        assert_eq!(value.load(Ordering::SeqCst), 42, "the initial value should be stored");

        signal.update_if(|v| {
            *v = 123;
            false
        });

        assert_eq!(signal.get(), 123, "signal value should be equal to the updated value");
        assert_eq!(call_count.load(Ordering::SeqCst), 1, "the effect should not be called for false condition");
        assert_eq!(value.load(Ordering::SeqCst), 42, "the stored value should not be changed");

        signal.update_if(|v| {
            *v = 321;
            true
        });

        assert_eq!(signal.get(), 321, "signal value should be equal to the updated value");
        assert_eq!(call_count.load(Ordering::SeqCst), 2, "the effect should be called for true condition");
        assert_eq!(value.load(Ordering::SeqCst), 321, "the stored value should be changed");
    }
}
// no-coverage:stop
