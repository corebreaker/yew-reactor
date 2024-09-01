mod component;

use component::*;
use yew_reactor::{
    components::{Reactor, IfTrue, IfFalse},
    hooks::use_reactor,
};
use yew::{prelude::*, Renderer};

#[function_component]
fn App() -> Html {
    let rt = use_reactor();
    let state = use_state(move || rt.create_signal(false));

    let toggle_action = {
        let state = state.clone();

        Callback::from(move |_| {
            let value = !(*state).get();

            (*state).set(value);
        })
    };

    let reset_action = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set((*state).runtime().create_signal(false));
        })
    };

    html! {
        <Container>
            <Block>
                <Button action={toggle_action} label={"Toggle"} />
                <Button action={reset_action} label={"Reset"} />
            </Block>

            <Block>
                <IfTrue<bool> when={(*state).clone()}>
                    <p class="tw-text-indigo-800">
                        {"The condition is "}<b class="tw-text-green-800">{"true"}</b>
                    </p>
                </IfTrue<bool>>
                <IfFalse<bool> when={(*state).clone()}>
                    <p class="tw-text-indigo-800">
                        {"The condition is "}<b class="tw-text-red-800">{"false"}</b>
                    </p>
                </IfFalse<bool>>
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
                    {"Example of use of the components `IfTrue` and `IfFalse`"}
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
