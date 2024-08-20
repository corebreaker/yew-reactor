use crate::{data::{DataList, Record}, card::SigCard};
use yew_reactor::{signal::Signal, duration::DurationInfo};
use yew::{Properties, Html, html, Component, Context};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq)]
pub enum Msg {
    Update(Vec<Uuid>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub values: Signal<DataList>,
    pub duration: Signal<DurationInfo>,
}

pub struct Panel {
    records: Vec<Uuid>,
    signals: HashMap<Uuid, Signal<Option<Record>>>,
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
                link.send_message(Msg::Update(list.with(|l| l.ids().collect())));
            }
        });

        Self {
            records: vec![],
            signals: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::Update(list) = msg {
            let values = ctx.props().values.clone();

            let old_id_list = self.records.iter().cloned().collect::<HashSet<_>>();
            let new_id_list = list.iter().copied().collect::<HashSet<_>>();

            // Records to remove
            for id in old_id_list.difference(&new_id_list) {
                self.signals.remove(id);
            }

            // Records to add
            let added_ids = new_id_list.difference(&old_id_list).cloned().collect::<HashSet<_>>();
            for id in &added_ids {
                let id_str = id.to_string();
                let signal = values.runtime().create_keyed_signal(values.clone(), &id_str);

                self.signals.insert(id.clone(), signal);
            }

            // Set the new list in the component
            self.records = list;
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.props().duration.with(|d| d.begin());

        let signals = self.signals.clone();
        let list = self.records.iter().map(|id| {
            html! {
                <SigCard key={id.to_string()} value={signals[id].clone()} />
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