use yew_reactor::{spawner::generators::TaskSpawner, signal::{Runtime, Signal}};
use cucumber::{given, then, when, World};
use cucumber_trellis::CucumberTest;
use std::{sync::{Arc, RwLock}, cell::{Cell, RefCell}};

#[derive(World, Default, Debug)]
pub(in super::super) struct Signals {
    rt: Option<Arc<Runtime>>,
    signal: Option<Signal<usize>>,
    signal_copy: Option<Signal<usize>>,
    signal_value: Option<usize>,
    call_count: Option<Arc<Cell<usize>>>,
    call_value: Option<Arc<Cell<usize>>>,
    expected_value: Option<usize>,
    dest_signal: Option<Signal<usize>>,
    other_signal: Option<Signal<usize>>,
}

impl Signals {
    fn rt(&self) -> Arc<Runtime> {
        self.rt.as_ref().cloned().expect("Runtime not set")
    }

    fn signal(&self) -> Signal<usize> {
        self.signal.as_ref().cloned().expect("Signal not set")
    }

    fn signal_copy(&self) -> Signal<usize> {
        self.signal_copy.as_ref().cloned().expect("Signal copy not set")
    }

    fn signal_value(&self) -> usize {
        self.signal_value.expect("Signal value not set")
    }

    fn call_count(&self) -> Arc<Cell<usize>> {
        self.call_count.as_ref().cloned().expect("Call count not set")
    }

    fn call_value(&self) -> Arc<Cell<usize>> {
        self.call_value.as_ref().cloned().expect("Call value not set")
    }

    fn expected_value(&self) -> usize {
        self.expected_value.as_ref().copied().expect("Expected value not set")
    }

    fn dest_signal(&self) -> Signal<usize> {
        self.dest_signal.as_ref().cloned().expect("Destination signal not set")
    }

    fn other_signal(&self) -> Signal<usize> {
        self.other_signal.as_ref().cloned().expect("Other signal not set")
    }
}

impl CucumberTest for Signals {
    const NAME: &'static str = "signals";
}

// Background: Signals are a created from a runtime instance

#[given(expr = "a created runtime instance")]
fn given_context(world: &mut Signals) {
    world.rt.replace(Runtime::new_with_spawn_generator(TaskSpawner::new()));
}

// Rule: A signal can be accessed from a copy, this is the right way to access the signal

#[given(expr = "a signal created from the runtime instance")]
fn given_signal(world: &mut Signals) {
    let rt = world.rt();

    world.signal.replace(Arc::clone(&rt).create_signal(42));
}

#[given(expr = "the/a copy of the signal(, the source signal [S])")]
fn given_signal_copy(world: &mut Signals) {
    world.signal_copy.replace(world.signal().clone());
}

#[when(expr = "a value is got from the signal")]
fn when_signal_value(world: &mut Signals) {
    world.signal_value.replace(world.signal_copy().get());
}

#[then(expr = "the value of the signal should be returned")]
fn then_signal_value(world: &mut Signals) {
    assert_eq!(world.signal_value(), 42);
}

#[when(expr = "a value is set to the (source )signal( S)")]
fn when_signal_set_value(world: &mut Signals) {
    world.signal_copy().set(24);
}

#[then(expr = "the value of the signal should be set")]
fn then_signal_set_value(world: &mut Signals) {
    assert_ne!(world.signal().get(), 42);
    assert_eq!(world.signal().get(), 24);
}

#[when(expr = "the value is got from the signal and transformed through a function")]
fn when_signal_value_transformed(world: &mut Signals) {
    world.signal_value.replace(world.signal_copy().with(|v| v * 2));
}

#[then(expr = "the transformed value should be returned")]
fn then_signal_value_is_transformed(world: &mut Signals) {
    assert_eq!(world.signal_value(), 84);
}

#[when(expr = "the value is set to the signal through a function")]
fn when_signal_value_set_through_function(world: &mut Signals) {
    world.signal_copy().update(|v| *v += 8);
}

#[then(expr = "the value of the signal should be set to the transformed value")]
fn then_signal_value_set_through_function(world: &mut Signals) {
    assert_eq!(world.signal().get(), 50);
}

#[given(expr = "another signal is created from the runtime instance")]
fn given_another_signal_from_runtime(world: &mut Signals) {
    world.signal.replace(world.rt().create_signal(11));
}

#[when(expr = "the two signals are combined through a function")]
fn when_signals_combined(world: &mut Signals) {
    world.signal_value.replace(world.signal().with_another(world.signal_copy(), |v1, v2| v1 * v2));
}

#[then(expr = "the combined value should be returned")]
fn then_combined_value(world: &mut Signals) {
    assert_eq!(world.signal_value(), 462);
}

// Rule: A signal can be subscribed to an effect

#[when(expr = "the signal is subscribed to a new effect created with the runtime instance")]
fn when_signal_subscribed_to_effect(world: &mut Signals) {
    let signal = world.signal_copy();
    let rt = world.rt();

    let call_count = Arc::new(Cell::new(0));
    let call_value = Arc::new(Cell::new(100));

    world.call_count.replace(Arc::clone(&call_count));
    world.call_value.replace(Arc::clone(&call_value));
    world.expected_value.replace(42);
    world.signal_value.replace(42);
    rt.create_effect(move || {
        call_count.set(call_count.get() + 1);
        call_value.set(signal.get());
    });
}

#[then(expr = "the effect is called")]
fn then_effect_is_called(world: &mut Signals) {
    assert_ne!(world.call_count().get(), 0, "the effect should be notified so the flag should be set");
    assert_eq!(world.signal().get(), world.signal_value(), "the signal value should be set");
    assert_eq!(
        world.call_value().get(),
        world.expected_value(),
        "the effect should be notified so the value should be set",
    );
}

#[then(expr = "the modification should notify the effect")]
fn then_modification_notifies_effect(world: &mut Signals) {
    assert_eq!(world.call_count().get(), 2, "the effect should be notified so the flag should be set");
    assert_eq!(world.signal().get(), world.signal_value(), "the signal value should be set");
    assert_eq!(
        world.call_value().get(),
        world.expected_value(),
        "the effect should be notified so the value should be set",
    );
}

#[given(expr = "the signal is subscribed to a new effect created with the runtime instance")]
fn given_signal_subscribed_to_effect(world: &mut Signals) {
    let signal = world.signal();
    let rt = world.rt();

    let call_count = Arc::new(Cell::new(0));
    let call_value = Arc::new(Cell::new(102));

    world.call_count.replace(Arc::clone(&call_count));
    world.call_value.replace(Arc::clone(&call_value));
    rt.create_effect(move || {
        call_count.set(call_count.get() + 1);
        call_value.set(signal.get());
    });
}

#[when(expr = "the signal changes its value with an untracked change")]
fn when_signal_changes_value_with_untracked_change(world: &mut Signals) {
    world.expected_value.replace(42);
    world.signal_value.replace(321);
    world.signal().untracked_update(|v| *v = 321);
}

#[then(expr = "the modification should not notify the effect")]
fn then_modification_does_not_notifie_effect(world: &mut Signals) {
    assert_eq!(world.call_count().get(), 1, "the effect should not be notified so the flag should not be set");
    assert_eq!(world.signal().get(), world.signal_value(), "the signal value should be set");
    assert_eq!(
        world.call_value().get(),
        world.expected_value(),
        "the effect should not be notified so the value should stay to the previous value",
    );
    assert_ne!(
        world.call_value().get(),
        world.signal_value(),
        "the effect should not be notified so the value should not be set",
    );
}

#[when(expr = "the signal changes its value")]
fn when_signal_set_its_value(world: &mut Signals) {
    world.expected_value.replace(123);
    world.signal_value.replace(123);
    world.signal().set(123);
}

#[when(expr = "the signal set a value that is the same as the previous value")]
fn when_signal_set_its_value_that_is_the_same_as_the_previous_value(world: &mut Signals) {
    world.expected_value.replace(42);
    world.signal_value.replace(42);
    world.signal().set(42);
}

#[when(expr = "the signal changes its value through a conditioned change that requests an update")]
fn when_signal_set_its_value_through_conditioned_change_that_request_update(world: &mut Signals) {
    world.expected_value.replace(123);
    world.signal_value.replace(123);
    world.signal().update_if(|v| {
        *v = 123;
        true
    });
}

#[when(expr = "the signal changes its value through a conditioned change that doesn't request an update")]
fn when_signal_set_its_value_through_conditioned_change_that_does_not_request_update(world: &mut Signals) {
    world.expected_value.replace(42);
    world.signal_value.replace(123);
    world.signal().update_if(|v| {
        *v = 123;
        false
    });
}

// Rule: A signal can be linked to another signal and the linked signal acts as the source signal

#[given(expr = "a new signal is created from the runtime instance, the destination signal [D]")]
fn given_new_signal(world: &mut Signals) {
    world.dest_signal.replace(world.rt().create_signal(10));
}

#[when(expr = "the new signal D is linked to the signal S")]
fn when_signal_linked_to_signal(world: &mut Signals) {
    world.dest_signal().link_to(&world.signal_copy());
}

#[then(expr = "the signal D should be updated when the value of the signal S changes")]
fn then_linked_signal_updated_when_signal_s_changes(world: &mut Signals) {
    assert_eq!(world.dest_signal().get(), 24);
}

#[when(expr = "a value is set to the signal D")]
fn when_set_value_to_signal_d(world: &mut Signals) {
    world.dest_signal().set(123456);
}

#[then(expr = "the signal S should be updated when the value of the signal D changes")]
fn then_source_signal_updated_when_signal_d_changes(world: &mut Signals) {
    assert_eq!(world.signal_copy().get(), 123456);
}

#[given(expr = "the new signal D is linked to the signal S")]
fn given_signal_linked_to_signal(world: &mut Signals) {
    world.dest_signal().link_to(&world.signal_copy());
}

#[given(expr = "an effect is created from the signal S")]
fn given_effect_created_from_signal_s(world: &mut Signals) {
    let signal = world.signal();
    let rt = world.rt();

    let call_count = Arc::new(Cell::new(0));
    let call_value = Arc::new(Cell::new(0));

    world.call_count.replace(Arc::clone(&call_count));
    world.call_value.replace(Arc::clone(&call_value));
    rt.create_effect(move || {
        call_count.set(call_count.get() + 1);
        call_value.set(signal.get());
    });
}

#[then(expr = "the effect should be called")]
fn then_effect_should_be_called(world: &mut Signals) {
    assert_eq!(world.call_count().get(), 2, "the effect should be notified so the flag should be set");
    assert_eq!(world.call_value().get(), 123456, "the effect should be notified so the value should be set");
}

#[given(expr = "another signal is created from the runtime instance, the first destination signal [D1]")]
fn given_another_signal_created_the_first_destination(world: &mut Signals) {
    world.dest_signal.replace(world.rt().create_signal(111));
}

#[given(expr = "another signal is created from the runtime instance, the second destination signal [D2]")]
fn given_another_signal_created_the_second_destination(world: &mut Signals) {
    world.other_signal.replace(world.rt().create_signal(222));
}

#[given(expr = "the first destination signal D1 is linked to the source signal S")]
fn given_first_destination_linked_to_source(world: &mut Signals) {
    world.dest_signal().link_to(&world.signal_copy());
}

#[when(expr = "the second destination signal D2 is linked to the source signal S")]
fn when_second_destination_linked_to_source(world: &mut Signals) {
    world.other_signal().link_to(&world.signal_copy());
}

#[then(expr = "the first destination signal D1 should be updated")]
fn then_first_destination_updated(world: &mut Signals) {
    assert_eq!(world.dest_signal().get(), 24);
}

#[then(expr = "the second destination signal D2 should be updated")]
fn then_second_destination_updated(world: &mut Signals) {
    assert_eq!(world.other_signal().get(), 24);
}

#[when(expr = "a link is created from the signal D, the new destination signal [H]")]
fn when_link_created_from_signal_d(world: &mut Signals) {
    world.other_signal.replace(world.dest_signal().create_link());
}

#[when(expr = "a value is set to this link, the signal H")]
fn when_value_set_to_link_h(world: &mut Signals) {
    world.other_signal().set(333);
}

#[then(expr = "the signal S should be updated")]
fn then_signal_s_updated(world: &mut Signals) {
    assert_eq!(world.signal_copy().get(), 333);
}

#[then(expr = "the signal D should be updated")]
fn then_signal_d_updated(world: &mut Signals) {
    assert_eq!(world.dest_signal().get(), 333);
}
