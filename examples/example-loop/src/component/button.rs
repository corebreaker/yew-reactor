use yew::{Properties, Callback, Html, html, function_component};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub action: Callback<()>,
    pub label:  String,
}

#[function_component(Button)]
pub fn container(props: &Props) -> Html {
    let class = [
        "tw-py-2",
        "tw-px-3",
        "tw-bg-white",
        "tw-text-indigo-600",
        "tw-text-sm",
        "tw-font-semibold",
        "tw-rounded-md",
        "tw-shadow-lg",
        "hover:tw-shadow-indigo-700/50",
        "active:tw-shadow-white/50",
        "focus:tw-outline-none",
    ];

    let label = props.label.clone();
    let onclick = {
        let action = props.action.clone();

        move |_| {
            action.emit(());
        }
    };

    html! {
        <button class={ class.join(" ") } {onclick}>{ label }</button>
    }
}
