use super::{message::Message, properties::ValueProps};
use crate::{
    signal::{Runtime, Signal},
    css::CssClasses,
};

use yew::{Component, Context, Html, html};
use std::{marker::PhantomData, sync::Arc};

pub(super) struct ValueState<P: ValueProps + 'static, C: Component<Properties = P, Message = Message>> {
    element: String,
    value:   Option<String>,
    class:   String,

    classes:      CssClasses,
    class_signal: Signal<String>,

    default_classes:      CssClasses,
    default_class_signal: Signal<String>,

    p: PhantomData<P>,
    m: PhantomData<C>,
}

impl<P: ValueProps + 'static, C: Component<Properties = P, Message = Message>> ValueState<P, C> {
    pub(super) fn create(rt: Arc<Runtime>, ctx: &Context<C>) -> Self {
        let props = ctx.props();
        let element = Self::build_element_value(props);
        let class_signal = Arc::clone(&rt).create_signal(String::new());
        let prop_signal = Arc::clone(&rt).create_signal(String::new());
        let classes = Arc::clone(&rt).create_css_classes();
        let default_classes = Arc::clone(&rt).create_css_classes();
        let default_class_signal = Arc::clone(&rt).create_signal(String::new());

        if let Some(prop_classes) = props.classes().cloned() {
            classes.link_to(&prop_classes);
        }

        if let Some(prop_class_signal) = props.class_signal() {
            class_signal.link_to(&prop_class_signal);
        }

        classes.register_class_signal(class_signal.clone());
        classes.register_class_signal(prop_signal);

        if let Some(class) = props.class() {
            classes.add(class.as_str());
        }

        {
            let scope = ctx.link().clone();
            let classes = classes.clone();

            rt.create_effect(move || {
                scope.send_message(Message::SetClass(classes.values()));
            });
        }

        Self {
            element,
            value: Default::default(),
            class: String::new(),
            classes,
            class_signal,
            default_classes,
            default_class_signal,
            p: PhantomData,
            m: PhantomData,
        }
    }

    pub(super) fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub(super) fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }

    pub(super) fn changed(&mut self, ctx: &Context<C>, old_props: &P) -> bool {
        let props = ctx.props();
        let mut changed = false;

        {
            let element = Self::build_element_value(props);
            if self.element != element {
                self.element = element;
                changed = true;
            }
        }

        {
            let classes = props.classes().cloned();
            if classes.as_ref() != old_props.classes() {
                match classes {
                    Some(classes) => {
                        self.classes.link_to(&classes);
                    }
                    None => {
                        self.classes.link_to(&self.default_classes);
                    }
                }
            }
        }

        {
            let class_signal = props.class_signal();
            if class_signal != old_props.class_signal() {
                match class_signal {
                    Some(class_signal) => {
                        self.class_signal.link_to(class_signal);
                    }
                    None => {
                        self.class_signal.link_to(&self.default_class_signal);
                    }
                }
            }
        }

        {
            let class = props.class();
            if class != old_props.class() {
                if let Some(class) = class {
                    self.class_signal.set(class.to_string());
                }
            }
        }

        changed
    }

    pub(super) fn update(&mut self, msg: Message) -> bool {
        match msg {
            Message::SetValue(v) => {
                let changed = self.value != v;
                if changed {
                    self.value = v;
                }

                changed
            }

            Message::SetClass(c) => {
                let changed = self.class != c;
                if changed {
                    self.class = c;
                }

                changed
            }
        }
    }

    pub(super) fn view(&self) -> Html {
        let value = self.value.clone().unwrap_or_default();
        let element = self.element.clone();
        let class = self.class.clone();

        html! {
            <@{element} class={class}>{value}</@>
        }
    }

    fn build_element_value(props: &P) -> String {
        props.element().map_or_else(|| "div".to_string(), |v| v.to_string())
    }
}
