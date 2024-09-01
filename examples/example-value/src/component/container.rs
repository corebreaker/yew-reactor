use yew::{Properties, Children, Html, html, function_component};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    html! {
        <div class="tw-container tw-space-y-8 tw-flex tw-flex-col tw-leading-16">
            { props.children.clone() }
        </div>
    }
}
