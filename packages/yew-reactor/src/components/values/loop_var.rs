use super::{
    super::{r#loop::LoopVar, LoopContext},
    state::ValueState,
    message::Message,
    properties::ValueProps,
};
use crate::{signal::Signal, css::CssClasses};
use yew::{AttrValue, Component, Context, Html, Properties};

#[derive(Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<AttrValue>,

    #[prop_or_default]
    pub class_signal: Option<Signal<String>>,

    #[prop_or_default]
    pub classes: Option<CssClasses>,

    #[prop_or_default]
    pub element: Option<AttrValue>,
}

impl ValueProps for Props {
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

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class
            && self.class_signal == other.class_signal
            && self.classes == other.classes
            && self.element == other.element
    }
}

impl Eq for Props {}

pub struct LoopValue<T: ToString + Clone + Default + PartialEq + 'static> {
    state: ValueState<Props, Self>,
    value: LoopVar<T>,
}

impl<T: ToString + Clone + Default + PartialEq + 'static> Component for LoopValue<T> {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let value = ctx.link().get_loop_var::<T>();
        let state = ValueState::create(value.runtime(), ctx);

        {
            let scope = ctx.link().clone();
            let value = value.clone();

            value.runtime().create_effect(move || {
                scope.send_message(Message::SetValue(Some(
                    value.with_value(|v| v.as_ref().map(|v| v.to_string()).unwrap_or_default()),
                )));
            });
        }

        Self {
            state,
            value,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state.update(msg)
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        self.value.with_value(|_| ());
        self.state.changed(ctx, old_props)
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.state.view()
    }
}
