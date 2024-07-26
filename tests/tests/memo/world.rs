use super::{
    super::runner::RunnerForTests,
    keyed_signal_kind::KeyedSignalKind,
    collection::Collection,
    function::Function,
    value::Value,
};

use yew_reactor::{signal::{Signal, Runtime}, spawner::generators::TaskSpawner};
use cucumber_trellis::CucumberTest;
use cucumber::{given, then, when, World};
use std::{cell::{RefCell, Cell}, sync::Arc};

const THE_KEY: &'static str = "the-key";

#[derive(World, Default, Debug)]
pub(in super::super::super) struct MemoFunctions {
    rt: Option<Arc<Runtime>>,
    signal: Option<Signal<String>>,
    value: Option<Arc<RefCell<String>>>,
    last_value: Option<Arc<RefCell<String>>>,
    function: Option<Function>,
    update_counter: Option<Arc<Cell<usize>>>,
    call_counter: Option<Arc<Cell<usize>>>,
    call_flag: Option<Arc<Cell<bool>>>,
    collection: Option<Signal<Collection>>,
    keyed_signal: Option<Signal<Option<Value>>>,
    keyed_value: Option<Value>,
    keyed_other_value: Option<Value>,
    keyed_signal_kind: KeyedSignalKind,
    effect_value: Option<Arc<RefCell<Value>>>,
}

impl CucumberTest for MemoFunctions {
    const NAME: &'static str = "memo";
}

impl MemoFunctions {
    fn rt(&self) -> Arc<Runtime> {
        self.rt.as_ref().cloned().expect("Runtime not set")
    }

    fn signal(&self) -> Signal<String> {
        self.signal.as_ref().cloned().expect("Signal not set")
    }

    fn value(&self) -> Arc<RefCell<String>> {
        self.value.as_ref().cloned().expect("Value signal not set")
    }

    fn last_value(&self) -> Arc<RefCell<String>> {
        self.last_value.as_ref().cloned().expect("Last value not set")
    }

    fn function(&self) -> Function {
        self.function.as_ref().cloned().expect("Function not set")
    }

    fn update_counter(&self) -> Arc<Cell<usize>> {
        self.update_counter.as_ref().cloned().expect("Update counter not set")
    }

    fn call_counter(&self) -> Arc<Cell<usize>> {
        self.call_counter.as_ref().cloned().expect("Call counter not set")
    }

    fn call_flag(&self) -> Arc<Cell<bool>> {
        self.call_flag.as_ref().cloned().expect("Call flag not set")
    }

    fn has_keyed_signal_kind(&self, kind: KeyedSignalKind) -> bool {
        self.keyed_signal_kind.has_kind(kind).expect("Keyed signal kind not set")
    }

    fn collection(&self) -> Signal<Collection> {
        self.collection.as_ref().cloned().expect("Collection signal not set")
    }

    fn keyed_signal(&self) -> Signal<Option<Value>> {
        self.keyed_signal.as_ref().cloned().expect("Keyed signal not set")
    }

    fn keyed_value(&self) -> Value {
        self.keyed_value.as_ref().cloned().expect("Keyed value not set")
    }

    fn keyed_other_value(&self) -> Value {
        self.keyed_other_value.as_ref().cloned().expect("Keyed other value not set")
    }

    fn effect_value(&self) -> Arc<RefCell<Value>> {
        self.effect_value.as_ref().cloned().expect("Effect value not set")
    }
}

// Background: Signals are a created from a runtime instance

#[given(expr = "a created runtime instance")]
fn given_context(world: &mut MemoFunctions) {
    world.rt.replace(Runtime::new().with_spawn_generator(TaskSpawner::new()).with_defer_runner(RunnerForTests));
}

// Rule: Creating a memo function will create a signal which will notify subscribers when the function returns changes

#[given(expr = "a signal is created from the runtime instance")]
fn given_signal(world: &mut MemoFunctions) {
    world.signal.replace(world.rt().create_signal(String::from("any-value")));
}

#[given(expr = "a function that returns a value")]
fn given_a_function_that_returns_a_value(world: &mut MemoFunctions) {
    let call_counter = Arc::new(Cell::new(0));
    let call_flag = Arc::new(Cell::new(false));

    world.call_counter.replace(Arc::clone(&call_counter));
    world.call_flag.replace(Arc::clone(&call_flag));

    let signal = world.signal();
    world.function.replace(Function::new(move |value: Option<&String>| {
        call_counter.set(call_counter.get() + 1);
        call_flag.set(value.is_none());

        signal.get()
    }));
}

#[when(expr = "the memo function is created")]
fn when_memo_function_is_created(world: &mut MemoFunctions) {
    let function = world.function().get();

    world.rt().create_memo(move |value| function(value));
}

#[then(expr = "the function is called to get the initial value")]
fn then_function_is_called_to_get_initial_value(world: &mut MemoFunctions) {
    assert!(world.call_flag().get(), "function should be firstly called with None");
    assert_eq!(world.call_counter().get(), 1, "function should be called once");
}

#[given(expr = "the signal created from a memo function with an initialized value which is computed from the signal")]
fn given_signal_created_from_memo_function(world: &mut MemoFunctions) {
    let update_counter = Arc::new(Cell::new(0));
    let call_counter = Arc::new(Cell::new(0));
    let call_flag = Arc::new(Cell::new(false));
    let last_value = Arc::new(RefCell::new(String::from("<No value>")));
    let arg_value = Some(String::from("any-value"));

    world.update_counter.replace(Arc::clone(&update_counter));
    world.call_counter.replace(Arc::clone(&call_counter));
    world.call_flag.replace(Arc::clone(&call_flag));
    world.last_value.replace(Arc::clone(&last_value));

    let signal = world.signal();
    let memo = world.rt().create_memo(move |value: Option<&String>| {
        call_counter.set(call_counter.get() + 1);
        call_flag.set(value == arg_value.as_ref());

        {
            let mut last_value = last_value.borrow_mut();

            match value.as_ref() {
                None => {
                    *last_value = String::from("<No value>");
                }
                Some(value) => {
                    last_value.push_str("/");
                    last_value.push_str(value);
                }
            }
        }

        signal.get()
    });

    let value = Arc::new(RefCell::new(String::from("")));

    world.value.replace(Arc::clone(&value));
    world.rt().create_effect(move || {
        let mut value = value.borrow_mut();

        update_counter.set(update_counter.get() + 1);
        *value = memo.get();
    });
}

#[when(expr = "the signal value has changed")]
fn when_signal_value_has_changed(world: &mut MemoFunctions) {
    world.signal().set(String::from("new-value"));
}

#[when(expr = "the same value is set to the signal")]
fn when_same_value_is_set_to_signal(world: &mut MemoFunctions) {
    world.signal().set(String::from("any-value"));
}

#[then(expr = "the function is called to get the value from the signal")]
fn then_function_is_called_to_get_value_from_signal(world: &mut MemoFunctions) {
    let last_value = world.last_value().borrow().clone();

    assert!(world.call_flag().get(), "function should be called with the initial value: last value={last_value}");
    assert_eq!(
        world.call_counter().get(),
        2,
        "function should be called twice, with the initial value and with the value set to the signal",
    );
}

#[then(expr = "the signal notifies its subscribers with the new value")]
fn then_signal_notifies_subscribers_with_new_value(world: &mut MemoFunctions) {
    assert_eq!(world.update_counter().get(), 2, "signal should notify its subscribers with the new value");
    assert_eq!(
        world.value().borrow().as_str(),
        "new-value",
        "signal should notify its subscribers with the new value",
    );
}

#[then(expr = "the signal does not notify its subscribers")]
fn then_signal_does_not_notify_subscribers(world: &mut MemoFunctions) {
    assert_eq!(world.update_counter().get(), 1, "signal should not notify its subscribers");
    assert_eq!(
        world.value().borrow().as_str(),
        "any-value",
        "signal should not notify its subscribers with the same value",
    );
}

// Rule: A keyed signal is a memo function which will notify subscribers
//       when the collection changes its value for a key

#[given(expr = "the value for the key in the collection used will not be stringified")]
fn given_value_for_key_in_collection_not_stringified(world: &mut MemoFunctions) {
    world.keyed_signal_kind = KeyedSignalKind::Normal;
}

#[given(expr = "the value for the key in the collection used will be stringified")]
fn given_value_for_key_in_collection_stringified(world: &mut MemoFunctions) {
    world.keyed_signal_kind = KeyedSignalKind::Stringified;
}

#[given(expr = "a signal is created from the runtime instance with a collection signal and a defined key")]
fn given_signal_created_from_collection_signal_with_defined_key(world: &mut MemoFunctions) {
    let collection = world.rt().create_signal(Collection::default());

    if world.has_keyed_signal_kind(KeyedSignalKind::Stringified) {
        let value = Value::String(String::from("any-value"));

        world.keyed_value.replace(value.clone());
        world.keyed_other_value.replace(Value::String(String::from("new-value")));
        collection.update(|c| {
            c.put(THE_KEY, value);
        });
    } else {
        let value = Value::Int(1234_i32);

        world.keyed_value.replace(value.clone());
        world.keyed_other_value.replace(Value::Int(5678_i32));
        collection.update(|c| {
            c.put(THE_KEY, value);
        });
    }

    world.collection.replace(collection.clone());
}

#[given(expr = "a keyed signal is created from the collection signal")]
fn given_keyed_signal_created_from_collection_signal(world: &mut MemoFunctions) {
    world.keyed_signal.replace(world.rt().create_keyed_signal(world.collection(), THE_KEY));
}

#[when(expr = "the keyed signal is created")]
fn when_keyed_signal_is_created(world: &mut MemoFunctions) {
    world.keyed_signal.replace(world.rt().create_keyed_signal(world.collection(), THE_KEY));
}

#[then(expr = "the initial value for the key in the collection si set to the collection signal")]
fn then_initial_value_for_key_in_collection_is_set_to_collection_signal(world: &mut MemoFunctions) {
    assert_eq!(
        world.keyed_signal().get(),
        Some(world.keyed_value()),
        "keyed signal should have the same value when the keyed signal is created",
    );
}

#[given(expr = "an effect created with the keyed signal")]
fn given_effect_created_with_keyed_signal(world: &mut MemoFunctions) {
    let effect_value = Arc::new(RefCell::new(Value::None));
    let call_counter = Arc::new(Cell::new(0));
    let keyed_signal = world.keyed_signal();

    world.effect_value.replace(Arc::clone(&effect_value));
    world.call_counter.replace(Arc::clone(&call_counter));
    world.rt().create_effect(move || {
        call_counter.set(call_counter.get() + 1);
        effect_value.replace(keyed_signal.get().unwrap_or(Value::None));
    });
}

#[when(expr = "the same value located at the key in the collection is set through the collection signal")]
fn when_same_value_located_at_key_in_collection_is_set_through_collection_signal(world: &mut MemoFunctions) {
    let value = world.keyed_value();

    world.collection().update(|c| {
        c.put(THE_KEY, value);
    });
}

#[then(expr = "the keyed signal does not notify its subscribers")]
fn then_keyed_signal_does_not_notify_subscribers(world: &mut MemoFunctions) {
    assert_eq!(world.call_counter().get(), 1, "keyed signal should not notify its subscribers");
    assert_eq!(
        world.effect_value().borrow().clone(),
        world.keyed_value(),
        "keyed signal should not notify its subscribers with the same value",
    );
}

#[when(expr = "the signal value located at the key in the collection has changed")]
fn when_signal_value_located_at_key_in_collection_has_changed(world: &mut MemoFunctions) {
    let value = world.keyed_other_value();

    world.collection().update(|c| {
        c.put(THE_KEY, value);
    });
}

#[then(expr = "the keyed signal notifies its subscribers with the new value")]
fn then_keyed_signal_notifies_subscribers_with_new_value(world: &mut MemoFunctions) {
    assert_eq!(world.call_counter().get(), 2, "keyed signal should notify its subscribers with the new value");
    assert_eq!(
        world.effect_value().borrow().clone(),
        world.keyed_other_value(),
        "keyed signal should notify its subscribers with the new value",
    );
}
