use super::{state::ValueState, message::Message, properties::ValueProps};
use crate::{
    signal::{Signal, KeyedCollection},
    css::CssClasses,
};

use yew::{AttrValue, Component, Context, Html, Properties};
use std::marker::PhantomData;

#[derive(Properties)]
pub struct Props<C: KeyedCollection + 'static> {
    pub values: Signal<C>,

    pub index: AttrValue,

    #[prop_or_default]
    pub class: Option<AttrValue>,

    #[prop_or_default]
    pub class_signal: Option<Signal<String>>,

    #[prop_or_default]
    pub classes: Option<CssClasses>,

    #[prop_or_default]
    pub element: Option<AttrValue>,
}

impl<C: KeyedCollection + 'static> ValueProps for Props<C> {
    fn class(&self) -> Option<&AttrValue> {
        self.class.as_ref()
    }

    fn class_signal(&self) -> Option<&Signal<String>> {
        self.class_signal.as_ref()
    }

    fn classes(&self) -> Option<&CssClasses> {
        self.classes.as_ref()
    }

    fn element(&self) -> Option<&AttrValue> {
        self.element.as_ref()
    }
}

impl<C: KeyedCollection + 'static> PartialEq for Props<C> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
            && self.class == other.class
            && self.class_signal == other.class_signal
            && self.classes == other.classes
            && self.element == other.element
    }
}

impl<C: KeyedCollection + 'static> Eq for Props<C> {}

pub struct Item<T: ToString + 'static, C: KeyedCollection<Value = T> + 'static> {
    state: ValueState<Props<C>, Self>,
    ty:    PhantomData<T>,
    c:     PhantomData<C>,
}

impl<T: ToString + 'static, C: KeyedCollection<Value = T> + 'static> Component for Item<T, C> {
    type Message = Message;
    type Properties = Props<C>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let state = ValueState::create(props.values.runtime(), ctx);

        Self {
            state,
            ty: PhantomData,
            c: PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state.update(msg)
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        let coll = props.values.clone();
        let key = props.index.clone().to_string();
        let signal = coll.runtime().create_keyed_str_signal(coll, &key);
        let mut changed = self.state.changed(ctx, old_props);

        {
            let value = signal.get();
            if self.state.value() != value.as_ref() {
                self.state.set_value(value);
                changed = true;
            }
        }

        {
            let scope = ctx.link().clone();

            signal.runtime().create_effect(move || {
                scope.send_message(Message::SetValue(signal.with(|v| v.as_ref().map(|v| v.to_string()))));
            });
        }

        changed
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.state.view()
    }
}
