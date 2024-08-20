use crate::{data::DataList, card::LoopCard};
use yew_reactor::{signal::Signal, components::For, duration::DurationInfo};
use yew::{Properties, Html, html, Component, Context};
use crate::data::Record;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub values: Signal<DataList>,
    pub duration: Signal<DurationInfo>,
}

pub struct Panel;

impl Component for Panel {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <ul>
                <For<Record, DataList> values={props.values.clone()} duration={props.duration.clone()}>
                    <LoopCard />
                </For<Record, DataList>>
            </ul>
       }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        ctx.props().duration.update(|d| {
            d.end();
        });
    }
}
