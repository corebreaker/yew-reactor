use yew::{Properties, Children, Html, html, function_component};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Block)]
pub fn block(props: &Props) -> Html {
    let class = [
        "tw-p-4",
        "tw-flex",
        "tw-flex-row",
        "tw-container",
        "tw-space-x-4",
        "tw-rounded-lg",
        "tw-bg-indigo-300",
        "tw-shadow-xl",
        "tw-drop-shadow-lg",
        "hover:tw-drop-shadow-xl",
    ];

    html! {
        <div class = { class.join(" ") }>
            { props.children.clone() }
        </div>
    }
}
