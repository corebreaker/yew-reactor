mod component;

use component::*;
use yew_reactor::{components::{Reactor, For, LoopValue}, hooks::use_reactor};
use yew::{prelude::*, Renderer};

#[function_component]
fn App() -> Html {
    let init = [
        "Line L001",
        "Line L002",
        "Line L003",
    ].into_iter().map(String::from).collect::<Vec<_>>();

    let rt = use_reactor();
    let state = {
        let init = init.clone();

        use_state(move || rt.create_signal(init))
    };

    let add_action = {
        let counter = use_state(|| 4_usize);
        let state = state.clone();

        Callback::from(move |_| {
            let value = *counter;
            counter.set(value + 1);

            (*state).update(|v| {
                v.push(format!("Line L{value:03}"));
            });
        })
    };

    let remove_action = {
        let state = state.clone();

        Callback::from(move |_| {
            (*state).update(|v| {
                if v.len() > 2 {
                    v.remove(1);
                }
            });
        })
    };

    let reset_action = {
        let state = state.clone();
        let init = init.clone();

        Callback::from(move |_| {
            state.set((*state).runtime().create_signal(init.clone()));
        })
    };

    html! {
        <Container>
            <Block>
                <Button action={add_action} label={"Add"} />
                <Button action={remove_action} label={"Remove"} />
                <Button action={reset_action} label={"Reset"} />
            </Block>

            <Block>
                <div class="tw-text-indigo-800">
                    <p class="tw-font-bold tw-underline tw-text-xl tw-mb-4">{"List of lines"}</p>
                    <ul class="tw-list-disc tw-list-inside">
                        <For<String, Vec<String>> values={(*state).clone()}>
                            <LoopValue<String> element="li"/>
                        </For<String, Vec<String>>>
                    </ul>
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
                    {"Example of use of the component `For`"}
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
