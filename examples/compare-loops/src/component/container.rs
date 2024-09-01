use yew::{Properties, Children, Html, html, function_component, classes};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let class = classes!(
        "tw-container",
        "tw-relative",
        "tw-h-full",
        "tw-justify-start",
        "tw-space-y-8",
        "tw-flex",
        "tw-flex-col",
        "tw-leading-16",
    );

    html! {
        <div {class}>
            { props.children.clone() }
        </div>
    }
}
