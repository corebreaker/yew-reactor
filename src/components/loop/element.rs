use super::LoopDataContext;
use crate::signal::Signal;
use yew::{context::ContextProvider, Children, Component, Context, Html, Properties, ToHtml, html};

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
    value: Signal<Option<T>>,
}

impl<T: Clone + Default + 'static> LoopElement<T> {
    pub(super) fn get_signal(&self) -> Signal<Option<T>> {
        self.value.clone()
    }
}

impl<T: Clone + Default + PartialEq + 'static> Component for LoopElement<T> {
    type Message = ();
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: ctx.props().value.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let updated = if old_props.value != ctx.props().value {
            self.value = ctx.props().value.clone();

            true
        } else {
            false
        };

        updated || ctx.props().children != old_props.children
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let children = ctx.props().children.to_html();
        let context = LoopDataContext::new(self.value.clone());

        html! {
            <ContextProvider<LoopDataContext<T>> {context}>
                {children}
            </ContextProvider<LoopDataContext<T>>>
        }
    }
}
