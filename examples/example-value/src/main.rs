mod component;

use component::*;
use yew_reactor::{
    components::{Reactor, Value},
    hooks::use_reactor,
};
use yew::{prelude::*, Renderer};

#[function_component]
fn App() -> Html {
    let rt = use_reactor();
    let state = use_state(move || rt.create_signal(0_usize));
    let inc_action = {
        let counter = state.clone();
        move |_| {
            let v = counter.get() + 1;

            (*counter).set(v);
        }
    };

    let reset_action = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set((*state).runtime().create_signal(0_usize));
        })
    };

    html! {
        <Container>
            <Block>
                <Button action={inc_action} label={"+1"} />
                <Button action={reset_action} label={"Reset"} />
            </Block>

            <Block>
                <div style="padding: 5px">
                    <p class="tw-text-indigo-800">
                        { "Value:" }
                        <Value<usize>
                            signal={(*state).clone()}
                            class="tw-text-fuchsia-500 tw-font-bold tw-ml-3"
                            element="span"
                        />
                    </p>
                </div>

            </Block>
        </Container>
    }
}

#[function_component]
fn Rt() -> Html {
    html! {
        <Reactor>
            <div class="tw-mx-auto">
                <h1 class="tw-text-4xl tw-font-bold tw-text-center tw-mb-8 tw-text-indigo-300">
                    {"Example of use of the component `Value`"}
                </h1>
            </div>
            <div class="tw-relative tw-overflow-auto tw-p-8">
                <App />
            </div>
        </Reactor>
    }
}

fn main() {
    Renderer::<Rt>::new().render();
}
