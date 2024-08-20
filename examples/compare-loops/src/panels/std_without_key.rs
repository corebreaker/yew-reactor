use gloo_console::log as console;
use crate::{data::DataList, card::RecCard};
use yew_reactor::{signal::Signal, duration::DurationInfo};
use yew::{Properties, Html, html, Component, Context};

#[derive(Clone, PartialEq)]
pub enum Msg {
    Update(DataList),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub values: Signal<DataList>,
    pub duration: Signal<DurationInfo>,
}

pub struct Panel {
    list: DataList,
}

impl Component for Panel {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let list = ctx.props().values.clone();

        list.runtime().create_effect({
            let list = list.clone();
            let link = ctx.link().clone();

            move || {
                console!("Do something with the list");
                link.send_message(Msg::Update(list.get()));
            }
        });

        Self {
            list: list.get(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::Update(list) = msg {
            self.list = list;
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.props().duration.with(|d| d.begin());

        let list = self.list.values().map(|value| {
            html! {
                <RecCard value={value.clone()} />
            }
        });

        html! {
            <ul>
                { for list }
            </ul>
       }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        ctx.props().duration.update(|d| {
            d.end();
        });
    }
}