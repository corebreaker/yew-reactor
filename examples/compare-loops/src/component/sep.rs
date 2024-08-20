use yew::{Html, html, function_component};

#[function_component(Separator)]
pub fn separator() -> Html {
    let class = [
        "tw-w-0",
        "tw-ring",
        "tw-ring-1",
        "tw-border",
        "tw-outline",
        "tw-outline-1",
        "tw-ring-indigo-400",
        "tw-outline-indigo-300",
        "tw-border-indigo-200",
    ];

    html! {
        <div class={ class.join(" ") }/>
    }
}
