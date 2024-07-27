use crate::signal::Signal;
use yew::{Children, Component, Context, Html, html, Properties};
use std::marker::PhantomData;
use std::sync::Arc;

pub enum ElementMsg {
    SetBody(Html),
    SetCondition(bool),
}

#[derive(Properties)]
pub(crate) struct Props<T: Clone + Default + 'static> {
    pub(crate) value:    Signal<Option<T>>,
    pub(crate) children: Children,
}

impl<T: Clone + Default + 'static> PartialEq for Props<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Clone + Default + 'static> Eq for Props<T> {}

pub(crate) struct LoopElement<T: Clone + Default + 'static> {
    condition: bool,
    signal:    Signal<T>,
    children:  Html,
    ty:        PhantomData<T>,
}

impl<T: Clone + Default + 'static> LoopElement<T> {
    fn make_body(value: &T, children: Children) -> Html {
        html! {
            {children}
        }
    }
}

impl<T: Clone + Default + 'static> Component for LoopElement<T> {
    type Message = ElementMsg;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let condition = props.value.with(|v| v.is_some());
        let (children, signal) = {
            let rt = props.value.runtime();
            let children = props.children.clone();

            props.value.with(move |v| match v {
                Some(v) => (Self::make_body(v, children), rt.create_signal(v.clone())),
                None => (html!(), rt.create_signal(T::default())),
            })
        };

        Self {
            condition,
            signal,
            children,
            ty: PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ElementMsg::SetCondition(condition) => {
                let changed = self.condition != condition;
                if changed {
                    self.condition = condition;
                }

                changed
            }

            ElementMsg::SetBody(body) => {
                let changed = self.children != body;
                if changed {
                    self.children = body;
                }

                changed
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        let when = props.value.clone();
        let children = props.children.clone();
        let changed = {
            let children = when.with(|v| match v {
                Some(v) => Self::make_body(v, children.clone()),
                None => html!(),
            });

            let changed = self.children != children;
            if changed {
                self.children = children;
            }

            changed
        };

        {
            let rt = when.runtime();
            let scope = ctx.link().clone();
            let value = Arc::clone(&rt).create_signal(T::default());

            {
                let scope = scope.clone();
                let value = value.clone();

                value.runtime().create_effect(move || {
                    let children = children.clone();

                    value.with(|v| {
                        scope.send_message(ElementMsg::SetBody(Self::make_body(v, children)));
                    });
                });
            }

            rt.create_effect(move || {
                let condition = when.with(|v| match v {
                    None => false,
                    Some(v) => {
                        value.set(v.clone());
                        true
                    }
                });

                scope.send_message(ElementMsg::SetCondition(condition));
            });
        }

        changed
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.condition {
            self.children.clone()
        } else {
            html!()
        }
    }
}
