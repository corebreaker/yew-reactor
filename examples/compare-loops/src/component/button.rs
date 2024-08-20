use crate::{button_group::ButtonGroup, action_kind::ActionKind};
use yew_reactor::{signal::Signal, css::CssClasses, components::ReactorContext};
use yew::{Properties, Callback, Html, html, Component, Context};
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub enum Msg {
    Click,
    UpdateCSS(String),
    Disabled(bool),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,

    #[prop_or_else(Callback::noop)]
    pub action: Callback<()>,

    #[prop_or_default]
    pub group: Option<Signal<ButtonGroup>>,

    #[prop_or_default]
    pub kind: Option<ActionKind>,

    #[prop_or_default]
    pub disabled: Option<Signal<bool>>,
}

pub struct Button {
    disabled: bool,
    css: String,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let runtime = ctx.runtime().unwrap_or_default();
        let classes = Arc::clone(&runtime).create_css_classes();

        classes.extend(vec![
            "tw-py-2",
            "tw-px-3",
            "tw-bg-white",
            "tw-text-indigo-600",
            "tw-text-sm",
            "tw-font-semibold",
            "tw-rounded-md",
            "focus:tw-outline-none",
            "active:tw-shadow-white/50",
        ]);

        let disabled = ctx.props().disabled.as_ref().map(|d| d.get()).unwrap_or_default();
        if disabled {
            setup_css_for_disabled(classes.clone());
        } else {
            setup_css_for_unselected(classes.clone());

            if let Some(group) = ctx.props().group.as_ref().cloned() {
                let classes = classes.clone();
                let kind = ctx.props().kind;

                if let Some(selection) = group.with(move |group| group.selection()) {
                    if let Some(kind) = kind {
                        if selection == kind {
                            setup_css_for_selected(classes);
                        }
                    }
                }
            }
        }

        if let Some(group) = ctx.props().group.as_ref().cloned() {
            let classes = classes.clone();
            if let Some(kind) = ctx.props().kind {
                Arc::clone(&runtime).create_effect(move || {
                    match group.with(|group| group.selection()) {
                        Some(selection) if selection == kind => {
                            setup_css_for_selected(classes.clone());
                        }
                        _ => {
                            setup_css_for_unselected(classes.clone());
                        }
                    }
                });
            }
        }

        if let Some(disabled_state) = ctx.props().disabled.as_ref().cloned() {
            let classes = classes.clone();
            let scope = ctx.link().clone();

            Arc::clone(&runtime).create_effect(move || {
                let classes = classes.clone();
                let scope = scope.clone();

                disabled_state.with(move |disabled| {
                    if *disabled {
                        setup_css_for_disabled(classes.clone());
                    } else {
                        setup_css_for_unselected(classes.clone());
                    }

                    scope.send_message(Msg::Disabled(*disabled));
                });
            });
        }

        runtime.create_effect({
            let classes = classes.clone();
            let scope = ctx.link().clone();

            move || {
                scope.send_message(Msg::UpdateCSS(classes.values()));
            }
        });

        let css = classes.values();

        Self {
            css,
            disabled: ctx.props().disabled.as_ref().map(|d| d.get()).unwrap_or_default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();

        match msg {
            Msg::Click => {
                if self.disabled {
                    return false;
                }

                match props.group.as_ref().cloned() {
                    None => {
                        props.action.emit(());
                    }
                    Some(group) => {
                        if let Some(kind) = props.kind {
                            group.update_if(|group| {
                                match group.selection() {
                                    Some(selection) if selection == kind => false,
                                    _ => {
                                        group.select(kind);
                                        true
                                    }
                                }
                            });
                        }
                    }
                }

                false
            }

            Msg::Disabled(disabled) => {
                self.disabled = disabled;
                false
            }

            Msg::UpdateCSS(css) => {
                self.css = css;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let label = ctx.props().label.clone();
        let onclick = {
            let scope = ctx.link().clone();

            move |_| {
                scope.send_message(Msg::Click);
            }
        };

        html! {
            <button class={ self.css.clone() } {onclick}>{ label }</button>
        }
    }
}

fn setup_css_for_disabled(classes: CssClasses) {
    classes.remove_from_iter(vec![
        "tw-shadow-lg",
        "tw-cursor-pointer",
        "tw-shadow-white/50",
        "hover:tw-shadow-indigo-700/50",
        "active:tw-shadow-white/50",
   ]);

    classes.extend(vec![
        "tw-shadow-xs",
        "tw-bg-white",
        "tw-opacity-50",
        "tw-cursor-not-allowed",
    ]);
}

fn setup_css_for_selected(classes: CssClasses) {
    classes.remove_from_iter(vec![
        "tw-opacity-50",
        "tw-shadow-lg",
        "tw-bg-white",
        "tw-cursor-pointer",
        "hover:tw-shadow-indigo-700/50",
        "active:tw-shadow-white/50",
    ]);

    classes.extend(vec![
        "tw-bg-indigo-200",
        "tw-shadow-white/50",
        "tw-cursor-not-allowed",
    ]);
}

fn setup_css_for_unselected(classes: CssClasses) {
    classes.remove_from_iter(vec![
        "tw-opacity-50",
        "tw-bg-indigo-200",
        "tw-shadow-white/50",
        "tw-cursor-not-allowed",
    ]);

    classes.extend(vec![
        "tw-shadow-lg",
        "tw-bg-white",
        "tw-cursor-pointer",
        "hover:tw-shadow-indigo-700/50",
        "active:tw-shadow-white/50",
    ]);
}
