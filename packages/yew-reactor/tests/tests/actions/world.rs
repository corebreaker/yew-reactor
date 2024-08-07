use super::{super::runner::RunnerForTests, function::Function, stall::Stall};
use yew_reactor::{
    spawner::{generators::TaskSpawner, LocalFuture},
    signal::Runtime,
    action::Action,
};

use cucumber_trellis::CucumberTest;
use cucumber::{given, then, when, World};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
    thread::sleep,
};

#[derive(World, Debug, Default)]
pub(in super::super::super) struct Actions {
    rt:     Option<Arc<Runtime>>,
    func:   Option<Function>,
    action: Option<Action<(), &'static str>>,
    stall:  Option<Arc<Stall>>,
    value:  Option<Arc<RwLock<String>>>,
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

    fn stall(&self) -> Arc<Stall> {
        self.stall.as_ref().cloned().expect("Stall not set")
    }

    fn value(&self) -> Arc<RwLock<String>> {
        self.value.as_ref().cloned().expect("Value not set")
    }
}

impl CucumberTest for Actions {
    const NAME: &'static str = "actions";
}

// Background: Signals are a created from a runtime instance

#[given(expr = "a created runtime instance")]
fn given_context(world: &mut Actions) {
    world.rt.replace(
        Runtime::new()
            .with_spawn_generator(TaskSpawner::new())
            .with_defer_runner(RunnerForTests),
    );
}

// Rule: An action must be created from an asynchronous function

#[given(expr = "an async function")]
fn given_async_func(world: &mut Actions) {
    world
        .func
        .replace(Function::new("hello", || LocalFuture::new(async { "hello" })));
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

// Rule: The async function is called by dispatching its action

#[given(expr = "an async function that returns a value")]
fn given_async_func_with_return_value(world: &mut Actions) {
    let stall = Arc::new(Stall::new());

    world.stall.replace(Arc::clone(&stall));
    world.func.replace(Function::new("value", move || {
        let stall = Arc::clone(&stall);

        LocalFuture::new(async move {
            stall.wait().await;

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
    world.stall().notify();
}

#[given(expr = "the action is dispatched")]
fn given_action_is_dispatched(world: &mut Actions) {
    world.action().dispatch(());
}

#[given(expr = "an effect is created from the signal stored in the action")]
fn given_effect_created_from_signal(world: &mut Actions) {
    let value = Arc::new(RwLock::new(String::new()));
    let action = world.action().value();

    world.value.replace(Arc::clone(&value));
    world.rt().create_effect(move || {
        if let Some(result) = action.with(|val_action| val_action.map(|val| val.to_string())) {
            let mut value = value.write().unwrap();

            *value = result;
        }
    });
}

#[when(expr = "the async function is executed")]
fn when_async_func_is_executed(world: &mut Actions) {
    world.stall().notify();
    while world.action().is_pending() {}
}

#[then(expr = "the stored value is the return value of the async function")]
fn then_stored_value_is_return_value(world: &mut Actions) {
    assert_eq!(
        world.action().get(),
        Some("value"),
        "the stored value should be the return value of the async function",
    );
}

// Rule: Actions can be used to trigger effects with the return value of the async function

#[when(expr = "the async function has been executed")]
fn when_async_func_has_been_executed(world: &mut Actions) {
    world.stall().notify();
    while world.action().is_pending() {
        sleep(Duration::from_millis(1));
    }
}

#[then(expr = "the return value of the async function is stored in the action")]
fn then_return_value_of_async_func_is_stored(world: &mut Actions) {
    assert_eq!(
        world.action().get(),
        Some("value"),
        r#"the stored value should be `Some("value")`"#
    );
}

#[then(expr = "the effect is notified with the return value of the async function")]
fn then_effect_is_notified_with_return_value(world: &mut Actions) {
    assert_eq!(
        world.value().read().unwrap().to_string(),
        String::from("value"),
        "the effect should be notified with the return value",
    );
}
