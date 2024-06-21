use yew_reactor::{signal::{Signal, Runtime}, spawner::generators::FuturesSpawner, css::CssClasses as YrCssClasses};
use cucumber_trellis::CucumberTest;
use cucumber::{given, then, when, World};
use std::{sync::Arc, cell::RefCell};

#[derive(World, Debug, Default)]
pub(in super::super) struct CssClasses {
    rt: Option<Arc<Runtime>>,
    rt_copy: Option<Arc<Runtime>>,
    instance: Option<YrCssClasses>,
    instance_copy: Option<YrCssClasses>,
    effect_value: Option<Arc<RefCell<String>>>,
    link: Option<YrCssClasses>,
    signal: Option<Signal<String>>,
    value: Option<String>,
    count: usize,
}

impl CssClasses {
    fn rt(&self) -> Arc<Runtime> {
        self.rt.as_ref().cloned().expect("Runtime not set")
    }

    fn rt_copy(&self) -> Arc<Runtime> {
        self.rt_copy.as_ref().cloned().expect("Runtime copy not set")
    }

    fn instance(&self) -> YrCssClasses {
        self.instance.as_ref().cloned().expect("Instance not set")
    }

    fn value(&self) -> String {
        self.value.as_ref().cloned().expect("Value not set")
    }

    fn instance_copy(&self) -> YrCssClasses {
        self.instance_copy.as_ref().cloned().expect("Instance copy not set")
    }

    fn effect_value(&self) -> Arc<RefCell<String>> {
        self.effect_value.as_ref().cloned().expect("Effect value not set")
    }

    fn link(&self) -> YrCssClasses {
        self.link.as_ref().cloned().expect("Link not set")
    }

    fn signal(&self) -> Signal<String> {
        self.signal.as_ref().cloned().expect("Signal not set")
    }
}

impl CucumberTest for CssClasses {
    const NAME: &'static str = "css";
}

#[given(expr = "a created runtime instance")]
fn given_context(world: &mut CssClasses) {
    world.rt.replace(Runtime::new_with_spawn_generator(FuturesSpawner));
}

#[given(expr = "the created runtime instance (copy of the reference to the runtime instance)")]
fn given_context_copy(world: &mut CssClasses) {
    world.rt_copy.replace(world.rt());
}

#[when(expr = "a CSS class is created from the runtime instance")]
fn when_create_css_class(world: &mut CssClasses) {
    world.instance.replace(world.rt().create_css_classes());
}

#[then(expr = "the CSS class instance is created")]
fn then_css_class_instance_created(world: &mut CssClasses) {
    assert_eq!(world.instance().values(), String::new(), "the value of CSS classes should be empty");
}

#[given(expr = "an instance of `CssClasses` created from the runtime instance")]
fn given_css_class_instance(world: &mut CssClasses) {
    world.instance.replace(world.rt().create_css_classes());
}

#[when(expr = "the value of the CSS classes is get")]
fn when_get_css_classes_value(world: &mut CssClasses) {
    world.value.replace(world.instance().values());
}

#[then(expr = "the value is empty")]
fn then_value_is_empty(world: &mut CssClasses) {
    assert_eq!(world.value(), "", "the value of CSS classes should be empty");
}

#[given(expr = "a copy of the instance of `CssClasses`")]
fn given_css_class_instance_copy(world: &mut CssClasses) {
    world.instance_copy.replace(world.instance().clone());
}

#[given(expr = "{int} CSS class(es) is added to the instance of `CssClasses`")]
fn given_css_classes_added(world: &mut CssClasses, count: usize) {
    world.count = count;

    let instance = world.instance_copy();

    for i in 0..count {
        instance.add(&format!("class{i:03}"));
    }
}

#[when(expr = "the value of the CSS class is get")]
fn when_get_css_classes_value_copy(world: &mut CssClasses) {
    world.value.replace(world.instance_copy().values());
}

#[then(expr = "the value is the list of CSS classes separated by a space")]
fn then_value_is_list_of_css_classes(world: &mut CssClasses) {
    let expected = (0..world.count)
        .map(|i| format!("class{i:03}"))
        .collect::<Vec<_>>()
        .join(" ");

    assert_eq!(
        world.value(),
        expected,
        "the value of CSS classes should be the list of CSS classes separated by a space",
    );
}

#[when(expr = "a CSS class is added to the instance of `CssClasses`")]
fn when_add_css_class(world: &mut CssClasses) {
    world.instance_copy().add("this-class");
}

#[then(expr = "the CSS class is added to the instance of `CssClasses`")]
fn then_css_class_is_added(world: &mut CssClasses) {
    assert!(world.instance_copy().contains("this-class"), "the CSS class should be added");
}

#[given(expr = "a CSS class is added to the instance of `CssClasses`")]
fn given_css_class_added(world: &mut CssClasses) {
    world.instance_copy().add("this-class");
}

#[when(expr = "the CSS class is removed from the instance of `CssClasses`")]
fn when_remove_css_class(world: &mut CssClasses) {
    world.instance_copy().remove("this-class");
}

#[then(expr = "the CSS class is removed from the instance of `CssClasses`")]
fn then_css_class_is_removed(world: &mut CssClasses) {
    assert!(!world.instance_copy().contains("this-class"), "the CSS class should be removed");
}

#[when(expr = "a CSS class is toggled from the instance of `CssClasses`")]
fn when_toggle_css_class(world: &mut CssClasses) {
    world.instance_copy().toggle("this-class");
}

#[when(expr = "the CSS class that was added is replaced with another CSS class")]
fn when_replace_css_class(world: &mut CssClasses) {
    world.instance_copy().replace("this-class", "that-class");
}

#[then(expr = "the CSS class is replaced, the old CSS class is removed and the new CSS class is added")]
fn then_css_class_is_replaced(world: &mut CssClasses) {
    assert!(!world.instance_copy().contains("this-class"), "the old CSS class should be removed");
    assert!(world.instance_copy().contains("that-class"), "the new CSS class should be added");
}

#[when(expr = "a CSS class is checked if it contains the CSS class")]
fn when_check_css_class_contains(world: &mut CssClasses) {
    let checked = if world.instance_copy().contains("this-class") {"contains"} else {"does not contain"};

    world.value.replace(String::from(checked));
}

#[then(expr = "the instance of `CssClasses` does not contain the CSS class")]
fn then_css_class_does_not_contain(world: &mut CssClasses) {
    assert_eq!(world.value(), "does not contain", "the CSS class should not be contained");
}

#[then(expr = "the instance of `CssClasses` contains the CSS class")]
fn then_css_class_contains(world: &mut CssClasses) {
    assert_eq!(world.value(), "contains", "the CSS class should be contained");
}

#[given(expr = "an effect created with this instance of `CssClasses` as a signal")]
fn given_effect_created_with_signal(world: &mut CssClasses) {
    let effect_value = Arc::new(RefCell::new(String::new()));
    world.effect_value.replace(Arc::clone(&effect_value));

    let classes = world.instance_copy();
    world.rt().create_effect(move || {
        let values = classes.values();
        let mut container = effect_value.borrow_mut();

        *container = values;
    });
}

#[then(expr = "the effect is notified")]
fn then_effect_is_notified(world: &mut CssClasses) {
    assert_eq!(
        world.effect_value().borrow().as_str(),
        world.instance_copy().values(),
        "the effect should be notified",
    );
}

#[given(expr = "a link is created from this instance of `CssClasses`")]
fn given_link_created(world: &mut CssClasses) {
    let link = world.rt().create_css_classes();

    link.link_to(&world.instance_copy());
    world.link.replace(link);
}

#[when(expr = "a new CSS class is added through the link")]
fn when_add_css_class_to_first(world: &mut CssClasses) {
    world.link().add("another-class");
}

#[then(expr = "the instance of `CssClasses` is notified")]
fn then_instance_is_notified(world: &mut CssClasses) {
    assert!(world.instance_copy().contains("another-class"), "the instance should be notified");
}

#[given(expr = "a signal attached to this instance of `CssClasses`")]
fn given_signal_attached(world: &mut CssClasses) {
    let signal = world.rt().create_signal(String::new());

    world.instance_copy().register_class_signal(signal.clone());
    world.signal.replace(signal);
}

#[when(expr = "the signal notifies a change to the CSS classes")]
fn when_signal_notifies_change(world: &mut CssClasses) {
    world.signal().set(String::from("the-class"));
}

#[then(expr = "the CSS classes have changed")]
fn then_css_classes_have_changed(world: &mut CssClasses) {
    assert!(world.instance_copy().contains("the-class"), "the CSS classes should have changed");
}

#[given(expr = "a CSS class is set through the signal")]
fn given_css_class_set_through_signal(world: &mut CssClasses) {
    world.signal().set(String::from("the-class"));
}

#[when(expr = "a new value is set through the signal")]
fn when_new_value_set_through_signal(world: &mut CssClasses) {
    world.signal().set(String::from("another-class"));
}

#[then(expr = "the old value is replaced by the new value in the CSS classes")]
fn then_old_value_replaced_by_new_value(world: &mut CssClasses) {
    assert!(!world.instance_copy().contains("the-class"), "the old value should be removed");
    assert!(world.instance_copy().contains("another-class"), "the new value should be added");
}
