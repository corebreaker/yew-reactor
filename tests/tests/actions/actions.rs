use super::function::Function;
use yew_reactor::{
    spawner::generators::{TaskSpawner, FuturesSpawner},
    spawner::LocalFuture,
    signal::Runtime,
    action::Action,
};

use cucumber_trellis::CucumberTest;
use cucumber::{given, then, when, World};
use std::{sync::{Arc, Condvar, Mutex}, cell::RefCell, future::Future};

#[derive(World, Debug, Default)]
pub(in super::super::super) struct Actions {
    rt: Option<Arc<Runtime>>,
    func: Option<Function>,
    action: Option<Action<(), &'static str>>,
    lock: Option<Arc<Mutex<()>>>,
    condition: Option<Arc<Condvar>>,
    value: Option<Arc<RefCell<String>>>,
}

impl Actions {
    fn rt(&self) -> Arc<Runtime> {
        self.rt.as_ref().cloned().expect("Runtime not set")
    }

    fn func(&self) -> Function {
        self.func.as_ref().cloned().expect("Function not set")
    }

    fn action(&self) -> &Action<(), &'static str> {
        self.action.as_ref().expect("Action not set")
    }

    fn lock(&self) -> Arc<Mutex<()>> {
        self.lock.as_ref().cloned().expect("Lock not set")
    }

    fn condition(&self) -> Arc<Condvar> {
        self.condition.as_ref().cloned().expect("Condition not set")
    }

    fn value(&self) -> Arc<RefCell<String>> {
        self.value.as_ref().cloned().expect("Value not set")
    }
}

impl CucumberTest for Actions {
    const NAME: &'static str = "actions";
}

#[given(expr = "a created runtime instance")]
fn given_context(world: &mut Actions) {
    world.rt.replace(Runtime::new_with_spawn_generator(FuturesSpawner));
}

#[given(expr = "an async function")]
fn given_async_func(world: &mut Actions) {
    world.func.replace(Function::new("hello", || LocalFuture::new(async { "hello" })));
}

#[when(expr = "an action is created from the async function")]
fn when_create_action_from_func(world: &mut Actions) {
    let func = world.func();

    world.action.replace(world.rt().create_action(move |_| func.call()));
}

#[then(expr = "the action is created with a pending state set as false")]
fn then_action_is_created_with_pending_state(world: &mut Actions) {
    assert!(!world.action().is_pending(), "the action should not be pending");
}

#[then(expr = "the value stored in the action is None")]
fn then_value_stored_in_action_is_none(world: &mut Actions) {
    assert_eq!(world.action().get(), None, "the stored value should be `None`");
}

#[given(expr = "an async function that returns a value")]
fn given_async_func_with_return_value(world: &mut Actions) {
    let condition = world.condition();
    let lock = world.lock();

    world.rt().spawner().set_generator(TaskSpawner::default());
    world.func.replace(Function::new("value", move || {
        let condition = Arc::clone(&condition);
        let lock = Arc::clone(&lock);

        LocalFuture::new(async move {
            let _lock = condition.wait(lock.lock().unwrap()).unwrap();

            "value"
        })
    }));
}

#[given(expr = "an action created from the async function")]
fn given_action_created_from_func(world: &mut Actions) {
    let func = world.func();

    world.action.replace(world.rt().create_action(move |_| func.call()));
}

#[when(expr = "the action is dispatched( again)")]
fn when_action_is_dispatched(world: &mut Actions) {
    world.action().dispatch(());
}

#[then(expr = "the pending state is set to true before the execution of the async function")]
fn then_pending_state_is_set_to_true_before_execution(world: &mut Actions) {
    assert!(world.action().is_pending(), "the action should be pending");
}

#[then(expr = "the pending state stay to true")]
fn then_pending_state_stay_to_true(world: &mut Actions) {
    assert!(world.action().is_pending(), "the action should be pending");
}

#[then(expr = "the stored value is None")]
fn then_stored_value_is_none(world: &mut Actions) {
    assert_eq!(world.action().get(), None, "the stored value should be `None`");
    world.condition().notify_all();
}

#[given(expr = "the action is dispatched")]
fn given_action_is_dispatched(world: &mut Actions) {
    world.action().dispatch(());
}

#[given(expr = "an effect is created from the signal stored in the action")]
fn given_effect_created_from_signal(world: &mut Actions) {
    let value = Arc::new(RefCell::new(String::new()));
    let action = world.action().value();

    world.value.replace(Arc::clone(&value));
    world.rt().create_effect(move || {
        if let Some(result) = action.with(|val_action| val_action.map(|val| val.to_string())) {
            let mut value = value.borrow_mut();

            *value = result;
        }
    });
}

#[when(expr = "the async function is executed")]
fn when_async_func_is_executed(world: &mut Actions) {
    world.condition().notify_all();
    while world.action().is_pending() {}
}

#[then(expr = "the return value of the async function is stored in the action")]
fn then_return_value_of_async_func_is_stored(world: &mut Actions) {
    assert_eq!(world.action().get(), Some("value"), r#"the stored value should be `Some("value")`"#);
}

#[then(expr = "the effect is notified with the return value of the async function")]
fn then_effect_is_notified_with_return_value(world: &mut Actions) {
    assert_eq!(
        world.value().borrow().to_string(),
        String::from("value"),
        "the effect should be notified with the return value",
    );
}
