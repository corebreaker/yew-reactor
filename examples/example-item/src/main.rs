mod component;

use component::*;
use yew_reactor::{
    components::{Reactor, Item},
    hooks::use_reactor,
};
use yew::{prelude::*, Renderer};

#[function_component]
fn App() -> Html {
    let rt = use_reactor();
    let state = use_state(move || rt.create_signal(vec![1_usize, 2, 3]));

    let reset_action = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set((*state).runtime().create_signal(vec![1_usize, 2, 3]));
        })
    };

    html! {
        <Container>
            <Block>
                {
                    (0..3)
                        .map(|idx| {
                            let label = format!("{}. +1", idx + 1);
                            let action = {
                                let counter = state.clone();
                                move |_| {
                                    (*counter).update(|list| list[idx] += 1);
                                }
                            };

                            html! {
                                <Button {action} {label} />
                            }
                        })
                        .collect::<Html>()
                }

                <Button action={reset_action} label={"Reset"} />
            </Block>

            {
                (0..3)
                    .map(|idx| {
                        let index = idx.to_string();

                        html! {
                            <Block>
                                <div style="padding: 5px">
                                    <p class="tw-text-indigo-800">
                                        { format!("Value {}:", idx + 1) }
                                        <Item<usize, Vec<_>>
                                            values={(*state).clone()}
                                            {index}
                                            class="tw-text-fuchsia-500 tw-font-bold tw-ml-3"
                                            element="span"
                                        />
                                    </p>
                                </div>
                            </Block>
                        }
                    })
                    .collect::<Html>()
            }
        </Container>
    }
}

#[function_component]
fn Rt() -> Html {
    html! {
        <Reactor>
            <div class="tw-mx-auto">
                <h1 class="tw-text-4xl tw-font-bold tw-text-center tw-mb-8 tw-text-indigo-300">
                    {"Example of use of the component `Item`"}
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
