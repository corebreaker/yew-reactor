use crate::{signal::Signal, components::AsBool};
use yew::{Component, Context, Html, Properties};
use std::marker::PhantomData;

pub enum Message {
    SetValue(bool),
}

#[derive(Properties)]
pub struct Props<T: AsBool + 'static> {
    pub when:     Signal<T>,
    pub children: Html,
}

impl<T: AsBool + 'static> PartialEq for Props<T> {
    fn eq(&self, other: &Self) -> bool {
        self.when == other.when
    }
}

impl<T: AsBool + 'static> Eq for Props<T> {}

pub struct ConditionState<T: AsBool + 'static, C: Component<Message = Message, Properties = Props<T>>> {
    condition: bool,
    signal:    Signal<T>,
    c:         PhantomData<C>,
}

impl<T: AsBool, C: Component<Message = Message, Properties = Props<T>>> ConditionState<T, C> {
    pub(super) fn create(ctx: &Context<C>) -> Self {
        let signal = ctx.props().when.clone();

        {
            let scope = ctx.link().clone();
            let condition = signal.clone();

            condition.runtime().create_effect(move || {
                let value = condition.with(AsBool::as_bool);

                scope.send_message(Message::SetValue(value));
            });
        }

        Self {
            condition: false,
            signal,
            c: PhantomData,
        }
    }

    pub(super) fn update(&mut self, msg: Message) -> bool {
        match msg {
            Message::SetValue(value) => {
                let changed = self.condition != value;
                if changed {
                    self.condition = value;
                }

                changed
            }
        }
    }

    pub(super) fn changed(&mut self, ctx: &Context<C>, _old_props: &Props<T>) -> bool {
        if ctx.props().when != self.signal {
            ctx.props().when.link_to(&self.signal);
        }

        false
    }

    pub(super) fn condition(&self) -> bool {
        self.condition
    }

    pub(super) fn view(&self, ctx: &Context<C>) -> Html {
        ctx.props().children.clone()
    }
}
