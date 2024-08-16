use super::{state::ValueState, message::Message, properties::ValueProps};
use crate::{
    signal::{Signal, KeyedCollection},
    css::CssClasses,
};

use yew::{AttrValue, Component, Context, Html, Properties};

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
    values: Signal<C>,
    signal: Signal<Option<String>>,
}

impl<T: ToString + 'static, C: KeyedCollection<Value = T> + 'static> Component for Item<T, C> {
    type Message = Message;
    type Properties = Props<C>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let values = props.values.clone();
        let state = ValueState::create(values.runtime(), ctx);

        let key = props.index.clone().to_string();
        let signal = values.runtime().create_keyed_str_signal(values.clone(), &key);

        {
            let scope = ctx.link().clone();
            let signal = signal.clone();

            signal.runtime().create_effect(move || {
                scope.send_message(Message::SetValue(signal.with(|v| v.as_ref().map(|v| v.to_string()))));
            });
        }

        Self {
            state,
            values,
            signal,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state.update(msg)
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().values != old_props.values {
            self.signal.with(|_| ());
            ctx.props().values.link_to(&self.values);
        }

        self.state.changed(ctx, old_props)
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.state.view()
    }
}
