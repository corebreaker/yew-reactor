use yew_reactor::signal::Signal;
use gloo_console as console;
use yew::{Properties, Component, Context, Html, html};

pub enum Msg {
    SetValue(usize),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub value: Signal<usize>,
}

#[derive(Clone, Default, PartialEq)]
pub struct Simple {
    value: usize,
    sig:   Option<Signal<usize>>,
}

impl Component for Simple {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().value.id();

        // Create an effect to update the value when the signal changes
        {
            let id = id.clone();
            let link = ctx.link().clone();
            let signal = ctx.props().value.clone();
            ctx.props().value.runtime().create_effect(move || {
                let v = signal.get();

                console::log!(format!("Effect Value {id:?}: {v}"));
                link.send_message(Msg::SetValue(v));
            });
        }

        console::log!(format!("Creating Value {id:?}"));
        Self {
            value: ctx.props().value.get(),
            sig:   Some(ctx.props().value.clone()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let id = ctx.props().value.id();

        console::log!(format!("Update Value {id:?}"));
        match msg {
            Msg::SetValue(value) => {
                console::log!(format!("Setting Value {id:?}: {value}"));
                self.value = value;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let id = ctx.props().value.id();

        console::log!(format!(
            "Changed Value {id:?}: {}",
            old_props.value != ctx.props().value
        ));

        match &self.sig {
            Some(sig) => {
                ctx.props().value.link_to(sig);
            }
            None => {
                self.sig.replace(ctx.props().value.clone());
            }
        }

        old_props.value != ctx.props().value
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div style="padding: 5px">
                <p class="tw-text-indigo-800">
                    { "Value:" }
                    <span class="tw-text-fuchsia-500 tw-font-bold tw-ml-3">
                        { self.value }
                    </span>
                </p>
            </div>
        }
    }
}
