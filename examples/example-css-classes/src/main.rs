mod component;

use component::*;
use yew_reactor::{components::{Reactor, Value}, hooks::use_reactor};
use yew::{prelude::*, Renderer};
use std::sync::Arc;

#[function_component]
fn App() -> Html {
    let rt = use_reactor();
    let classes = Arc::clone(&rt).create_css_classes();
    let css = Arc::clone(&rt).create_signal(String::from("tw-underline"));
    let state = use_state(move || rt.create_signal(12345_usize));

    let set_class_underline_action = {
        let css = css.clone();

        Callback::from(move |_| {
            css.set(String::from("tw-underline"));
        })
    };

    let set_class_overline_action = {
        let css = css.clone();

        Callback::from(move |_| {
            css.set(String::from("tw-overline"));
        })
    };

    let add_border_action = {
        let classes = classes.clone();

        Callback::from(move |_| {
            classes.add("tw-border");
        })
    };

    let remove_border_action = {
        let classes = classes.clone();

        Callback::from(move |_| {
            classes.remove("tw-border");
        })
    };

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
                <Button action={set_class_underline_action} label={"Underline"} />
                <Button action={set_class_overline_action} label={"Overline"} />
                <div class="tw-border tw-border-indigo-800 tw-w-0"/>
                <Button action={add_border_action} label={"Add border"} />
                <Button action={remove_border_action} label={"Remove border"} />
                <div class="tw-border tw-border-indigo-800 tw-w-0"/>
                <Button action={inc_action} label={"+1"} />
                <Button action={reset_action} label={"Reset"} />
            </Block>

            <Block>
                <div style="padding: 5px">
                    <p class="tw-text-indigo-800">
                        { "Value:" }
                        <Value<usize>
                            signal={(*state).clone()}
                            class="tw-text-fuchsia-500 tw-font-bold tw-ml-3 tw-border-indigo-800"
                            class_signal={css.clone()}
                            {classes}
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
                    {"Example of use of the CSS Classes"}
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
