use crate::data::Record;
use yew_reactor::signal::Signal;
use yew::{Component, Context, Html, html};
use yew_reactor::components::LoopContext;

pub enum Msg {
    Update(Option<Record>),
}

pub struct LoopCard {
    record: Record
}

impl Component for LoopCard {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let loop_var = ctx.get_loop_var::<Record>();

        loop_var.runtime().create_effect({
            let loop_var = loop_var.clone();
            let scope = ctx.link().clone();

            move || {
                scope.send_message(Msg::Update(loop_var.get_value()));
            }
        });

        LoopCard {
            record: Record::default()
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
