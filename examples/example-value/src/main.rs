use std::any::Any;
use yew::{prelude::*, Renderer};
use gloo_console as console;

#[derive(PartialEq, Properties)]
struct ValProps {
    children: Children,
}

#[derive(Clone, Default, PartialEq)]
struct Value {}

impl Component for Value {
    type Message = ();
    type Properties = ValProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.type_id();

        console::log!(format!("Creating Value {id:?}"));
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let id = ctx.type_id();

        console::log!(format!("Update Value {id:?}"));
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let id = ctx.type_id();

        console::log!(format!(
            "Changed Value {id:?}: {}",
            old_props.children != ctx.props().children
        ));
        old_props.children != ctx.props().children
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="padding: 5px">
                <p>{ "See Value" }</p>
                <p>{ ctx.props().children.clone() }</p>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let id = ctx.type_id();

        console::log!(format!("Rendering Value {id:?}: {first_render}"));
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        let id = ctx.type_id();

        console::log!(format!("Destroying Value {id:?}"));
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            console::log!(format!("Rendering App {value}"));
            counter.set(value);
        }
    };

    html! {
        <div>
            <div class="box">
                <button {onclick}>{ "+1" }</button>
            </div>
            <div class="box">
                <Value>
                    <p>{ *counter }</p>
                </Value>
            </div>
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
