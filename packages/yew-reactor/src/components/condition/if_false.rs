use super::state::{ConditionState, Props, Message};
use crate::components::AsBool;
use yew::{Component, Context, Html, html};

pub struct IfFalse<T: AsBool + 'static>(ConditionState<T, Self>);

impl<T: AsBool + 'static> Component for IfFalse<T> {
    type Message = Message;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self(ConditionState::create(ctx))
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.0.update(msg)
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        self.0.changed(ctx, old_props)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.0.condition() {
            html!()
        } else {
            self.0.view(ctx)
        }
    }
}
