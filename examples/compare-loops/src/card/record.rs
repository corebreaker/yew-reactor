use crate::data::Record;
use yew::{Properties, Component, Context, Html, html};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: Record,
}

pub struct RecCard {}

impl Component for RecCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        RecCard {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li>
                <span>{ ctx.props().value.first_name() }</span>
                <span class="tw-ml-1">{ ctx.props().value.last_name() }</span>
                <span class="tw-ml-1">{ format!("({})", &ctx.props().value.occupation()[..3]) }</span>
            </li>
        }
    }
}
