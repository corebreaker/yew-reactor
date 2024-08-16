use super::{state::ValueState, message::Message, properties::ValueProps};
use crate::{signal::Signal, css::CssClasses};
use yew::{AttrValue, Component, Context, Html, Properties};

#[derive(Properties)]
pub struct Props<T: ToString + 'static> {
    pub signal: Signal<T>,

    #[prop_or_default]
    pub class: Option<AttrValue>,

    #[prop_or_default]
    pub class_signal: Option<Signal<String>>,

    #[prop_or_default]
    pub classes: Option<CssClasses>,

    #[prop_or_default]
    pub element: Option<AttrValue>,
}

impl<T: ToString + 'static> ValueProps for Props<T> {
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

impl<T: ToString + 'static> PartialEq for Props<T> {
    fn eq(&self, other: &Self) -> bool {
        self.signal == other.signal
            && self.class == other.class
            && self.class_signal == other.class_signal
            && self.classes == other.classes
            && self.element == other.element
    }
}

impl<T: ToString + 'static> Eq for Props<T> {}

pub struct Value<T: ToString + 'static> {
    state: ValueState<Props<T>, Self>,
    signal: Signal<T>,
}

impl<T: ToString + 'static> Component for Value<T> {
    type Message = Message;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let state = ValueState::create(ctx.props().signal.runtime(), ctx);
        let signal = ctx.props().signal.clone();

        {
            let scope = ctx.link().clone();
            let signal = signal.clone();

            signal.runtime().create_effect(move || {
                scope.send_message(Message::SetValue(Some(signal.with(|v| v.to_string()))));
            });
        }

        Self {
            state,
            signal,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state.update(msg)
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().signal != old_props.signal {
            ctx.props().signal.link_to(&self.signal);
        }

        self.state.changed(ctx, old_props)
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.state.view()
    }
}
