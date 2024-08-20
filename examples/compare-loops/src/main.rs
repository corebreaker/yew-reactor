mod button_group;
mod component;
mod panels;
mod data;
mod card;
mod app;
mod panel_state;
mod action_kind;

use yew_reactor::components::Reactor;
use yew::{prelude::*, Renderer};

#[function_component(Front)]
fn front() -> Html {
    let title = use_state(|| String::from("Compare loops"));
    let class = classes!(
        "tw-container",
        "tw-flex",
        "tw-flex-col",
        "tw-justify-start",
        "tw-h-screen",
        "tw-min-h-screen",
        "tw-select-none",
        "tw-overflow-hidden",
    );

    html! {
        <div {class}>
            <div class="tw-container tw-flex tw-flex-col tw-mx-auto tw-flex-none">
                <h1 class="tw-text-4xl tw-font-bold tw-text-center tw-mb-2 tw-text-indigo-300">
                    {(*title).clone()}
                </h1>
            </div>
            <div class="tw-grow tw-overflow-hidden tw-p-8">
                <app::App />
            </div>
        </div>
    }
}

#[function_component(Main)]
fn main_component() -> Html {
    html! {
        <Reactor>
            <Front />
        </Reactor>
    }
}

fn main() {
    Renderer::<Main>::new().render();
}
