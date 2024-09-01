use yew::{Properties, Children, Html, html, AttrValue, classes, function_component};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: AttrValue,

    #[prop_or_default]
    pub fill: bool,

    pub children: Children,
}

#[function_component(Block)]
pub fn block(props: &Props) -> Html {
    let mut outer_class = classes!(
        "tw-container",
        "tw-flex",
        "tw-flex-col",
        "tw-p-0",
        "tw-relative",
        "tw-rounded-lg",
        "tw-bg-indigo-300",
        "tw-drop-shadow-lg",
        "tw-shadow-xl",
        "tw-overflow",
        "tw-overscroll-contain",
        "tw-overflow-x-auto",
        "hover:tw-drop-shadow-xl",
    );

    let inner_class = classes!(
        "tw-p-4",
        "tw-h-full",
        "tw-flex",
        "tw-flex-row",
        "tw-flex-nowrap",
        "tw-container",
        "tw-space-x-4",
        "tw-overflow-x-auto",
        "tw-overflow-y-hidden",
        "tw-overscroll-contain",
    );

    {
        let mut flex = "tw-flex-none";
        if props.fill {
            flex = "tw-grow";
        }

        outer_class.push(flex);
    }

    html! {
        <div class={ outer_class }>
            <div class="tw-flex-none">
                <h1 class="tw-text-2xl tw-font-bold tw-text-center tw-text-white">
                    { props.title.clone() }
                </h1>
            </div>
            <div class={ inner_class }>
                { props.children.clone() }
            </div>
        </div>
    }
}
