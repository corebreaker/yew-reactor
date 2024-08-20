use crate::data::Record;
use yew_reactor::signal::Signal;
use yew::{Properties, Component, Context, Html, html};

pub enum Msg {
    Update(Option<Record>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: Signal<Option<Record>>,
}

pub struct SigCard {
    record: Record,
}

impl Component for SigCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().value.runtime().create_effect({
            let signal = ctx.props().value.clone();
            let scope = ctx.link().clone();

            move || {
                scope.send_message(Msg::Update(signal.get()));
            }
        });

        SigCard {
            record: Record::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::Update(Some(record)) = msg {
            self.record = record;
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let occ = self.record.occupation();

        html! {
            <div>
                <span>{ self.record.first_name() }</span>
                <span class="tw-ml-1">{ self.record.last_name() }</span>
                <span class="tw-ml-1">{ format!("({})", &occ[..occ.len().min(3)]) }</span>
            </div>
        }
    }
}
