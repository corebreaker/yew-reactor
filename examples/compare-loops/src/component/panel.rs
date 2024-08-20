use super::Button;
use crate::{button_group::ButtonGroup, data::DataList};
use yew_reactor::{signal::Signal, components::Value, duration::DurationInfo};
use yew::{Properties, Children, Html, html, AttrValue, Callback, function_component, classes};
use std::rc::Rc;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: AttrValue,

    #[prop_or_default]
    pub subtitle: Option<AttrValue>,

    pub group: Signal<ButtonGroup>,

    pub action: Callback<()>,

    pub list: Signal<DataList>,

    pub disabled: Signal<bool>,

    pub duration: Signal<DurationInfo>,

    pub children: Children,
}

#[function_component(Panel)]
pub fn panel(props: &Props) -> Html {
    let main_class = classes!(
        "tw-container",
        "tw-flex",
        "tw-flex-col",
        "tw-p-0",
        "tw-h-full",
        "tw-bg-white",
        "tw-container",
        "tw-shadow-xl",
        "tw-rounded-lg",
        "tw-drop-shadow-lg",
        "tw-overflow-hidden",
        "hover:tw-drop-shadow-xl",
        "tw-min-w-[200px]",
    );

    let title_class = classes!(
        "tw-container",
        "tw-flex",
        "tw-flex-col",
        "tw-p-1",
        "tw-text-indigo-400",
        "tw-text-center",
        "tw-h-[110px]",
        "tw-min-h-[110px]",
        "tw-max-h-[110px]",
    );

    let button_class = classes!(
        "tw-flex-none",
        "tw-bg-indigo-200",
        "tw-p-2",
        "tw-text-center",
        "tw-grid",
        "tw-grid-cols-1",
        "tw-justify-items-center",
        "tw-content-center",
        "tw-h-[55px]",
        "tw-min-h-[55px]",
        "tw-max-h-[55px]",
    );

    let content_class = classes!(
        "tw-grow",
        "tw-border-t-2",
        "tw-border-indigo-50",
        "tw-h-full",
        "tw-overflow-auto",
        "tw-overscroll-contain",
    );

    let mut name = props.title.to_string();
    let subtitle = match &props.subtitle {
        Some(subtitle) => {
            name += " ";
            name += &subtitle.to_string();

            html! {
                <h2 class="tw-grow tw-font-bold tw-text-xs tw-text-ellipsis tw-mt-1 tw-mb-1">
                    { subtitle.clone() }
                </h2>
            }
        },
        None => {
            html! {}
        },
    };

    let format: Rc<dyn for<'a> Fn(&'a DataList) -> String> = Rc::new(|list| {
        format!("Rendered {} in", list.to_string())
    });

    html! {
        <div class={ main_class } title={ name.clone() }>
            <div class={ title_class }>
                <h1 class="tw-flex-none tw-font-bold tw-text-xl tw-text-ellipsis tw-leading-5">
                    { props.title.clone() }
                </h1>
                { subtitle }
                <Value<DataList>
                    class="tw-flex-none tw-text-xs tw-border-indigo-400 tw-border-t"
                    signal={ props.list.clone() }
                    {format}
                />
                <Value<DurationInfo>
                    class="tw-flex-none tw-font-bold tw-text-xs"
                    signal={ props.duration.clone() }
                />
            </div>
            <div class={ button_class }>
                <Button
                    label="Apply Action"
                    action={ props.action.clone() }
                    disabled={ props.disabled.clone() }
                />
            </div>
            <div class="tw-h-0 tw-border tw-border-indigo-100"/>
            <div class={ content_class }>
                <div class="tw-p-2">
                    { props.children.clone() }
                </div>
            </div>
        </div>
    }
}
