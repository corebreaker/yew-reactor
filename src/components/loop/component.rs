use super::element::LoopElement;
use crate::signal::{KeyedCollection, Signal};
use yew::{Component, Context, Html, Properties, Children, html};
use std::marker::PhantomData;

pub enum Msg {
    SetValues(Vec<Html>),
}

#[derive(Properties)]
pub struct Props<C: KeyedCollection> {
    pub(crate) values:   Signal<C>,
    pub(crate) children: Children,
}

impl<C: KeyedCollection> PartialEq for Props<C> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<C: KeyedCollection> Eq for Props<C> {}

pub struct For<T: Clone + PartialEq + Default + 'static, C: KeyedCollection> {
    values: Vec<Html>,
    t:      PhantomData<T>,
    c:      PhantomData<C>,
}

impl<T: Default + Clone + PartialEq + 'static, C: KeyedCollection<Value = T>> For<T, C> {
    fn make_item(children: &Children, values: Signal<C>, key: &str) -> Html {
        let value = values.runtime().create_keyed_signal(values, key);
        let key = key.to_string();

        html! {
            <LoopElement<T> {key} {value}>
                {children.clone()}
            </LoopElement<T>>
        }
    }

    fn make_values(children: &Children, values: Signal<C>) -> Vec<Html> {
        let list = values.clone();

        values.with(|c| {
            c.iter_keys()
                .map(|k| Self::make_item(children, list.clone(), &k))
                .collect::<Vec<_>>()
        })
    }
}

impl<T: Default + Clone + PartialEq + 'static, C: KeyedCollection<Value = T>> Component for For<T, C> {
    type Message = Msg;
    type Properties = Props<C>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            values: Self::make_values(&ctx.props().children, ctx.props().values.clone()),
            t:      PhantomData,
            c:      PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetValues(vals) => {
                self.values = vals;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let values = Self::make_values(&ctx.props().children, ctx.props().values.clone());
        Component::update(self, ctx, Msg::SetValues(values))
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                {self.values.clone()}
            </>
        }
    }
}
