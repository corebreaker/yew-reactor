use super::{id::SignalId, Runtime};
use std::{
    fmt::{Display, Debug, Formatter, Result as FmtResult},
    sync::atomic::{AtomicBool, Ordering},
    marker::PhantomData,
    sync::Arc,
};

pub struct SignalMap<S: 'static, R: 'static> {
    runtime:    Arc<Runtime>,
    id:         SignalId,
    mapper:     Arc<dyn for<'a> Fn(&'a S) -> R + 'static>,
    registered: AtomicBool,
    source_ty:  PhantomData<S>,
    result_ty:  PhantomData<R>,
}

impl<S: 'static, R: 'static> SignalMap<S, R> {
    pub(super) fn new<F>(runtime: Arc<Runtime>, id: SignalId, f: F) -> Self
    where
        for<'a> F: Fn(&'a S) -> R + 'static, {
        runtime.inc_signal_ref(id);

        Self {
            runtime,
            id,
            mapper: Arc::new(f),
            registered: AtomicBool::new(false),
            source_ty: PhantomData,
            result_ty: PhantomData,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn runtime(&self) -> Arc<Runtime> {
        Arc::clone(&self.runtime)
    }

    pub fn with<O, F: FnOnce(&R) -> O>(&self, f: F) -> O {
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
        let signal_value = value.downcast_ref::<S>().unwrap();
        let mapper = Arc::clone(&self.mapper);

        // return value
        let mapped_value = mapper(signal_value);

        f(&mapped_value)
    }

    pub fn with_another<XS, XR, O, F>(&self, other: SignalMap<XS, XR>, f: F) -> O
    where
        XS: 'static,
        XR: 'static,
        F: FnOnce(&R, &XR) -> O, {
        let other = other.clone();

        self.with(move |v| other.with(|o| f(v, o)))
    }
}

impl<S: 'static, R: 'static> Display for SignalMap<S, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "SignalMap[{}]", self.id.id())
    }
}

impl<S: 'static, R: Debug + 'static> Debug for SignalMap<S, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let id = self.id.id();

        self.with(|v| write!(f, "SignalMap[{id} = {v:?}]"))
    }
}

impl<S: 'static, R: Clone + 'static> SignalMap<S, R> {
    pub(crate) fn peek(&self) -> R {
        let runtime = self.runtime();

        // get value
        let signal_id = runtime.get_source_id(self.id);
        let value_ref = runtime.get_value(&signal_id);
        let value = value_ref.read().unwrap();
        let mapper = Arc::clone(&self.mapper);

        // return value
        mapper(value.downcast_ref::<S>().unwrap())
    }
}

impl<S: 'static, R: Clone + 'static> SignalMap<S, R> {
    pub fn get(&self) -> R {
        self.with(R::clone)
    }
}

impl<S: 'static, R: 'static> PartialEq for SignalMap<S, R> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<S: 'static, R: 'static> Eq for SignalMap<S, R> {}

impl<S: 'static, R: 'static> Clone for SignalMap<S, R> {
    fn clone(&self) -> Self {
        let runtime = Arc::clone(&self.runtime);
        runtime.inc_signal_ref(self.id);

        Self {
            runtime,
            id: self.id,
            mapper: Arc::clone(&self.mapper),
            registered: AtomicBool::new(false),
            source_ty: PhantomData,
            result_ty: PhantomData,
        }
    }
}

impl<S: 'static, R: 'static> Drop for SignalMap<S, R> {
    fn drop(&mut self) {
        self.runtime.clean_signal(self.id);
    }
}

unsafe impl<S: 'static, R: 'static> Send for SignalMap<S, R> {}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::tests::create_runtime;

    impl<S: 'static, T: 'static> SignalMap<S, T> {
        pub(in super::super) fn signal_id(&self) -> SignalId {
            self.id
        }
    }

    #[test]
    fn test_signal_map_get() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(42);

        assert_eq!(signal.get(), 42, "signal value should be equal to the initial value");
    }

    #[test]
    fn test_signal_map_with() {
        let rt = create_runtime();
        let signal = Arc::clone(&rt).create_signal(21);

        assert_eq!(
            signal.with(|v| *v * 2),
            42,
            "signal value should be equal to the initial value"
        );
    }

    #[test]
    fn test_combine_signal_map() {
        let rt = create_runtime();
        let signal1 = Arc::clone(&rt).create_signal(1);
        let signal2 = Arc::clone(&rt).create_signal(2);
        let result = signal1.with_another(signal2, |v1, v2| v1 + v2);

        assert_eq!(result, 3, "signal value should be equal to the sum of the two signals");
    }
}
// no-coverage:stop
