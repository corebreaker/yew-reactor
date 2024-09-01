use super::element::LoopElement;
use crate::signal::{KeyedCollection, Signal};
use yew::{Component, Context, Html, Properties, Children, html};
use std::marker::PhantomData;

pub enum Msg {
    Update,
}

#[derive(Properties)]
pub struct Props<C: KeyedCollection> {
    pub values:   Signal<C>,
    pub children: Children,

    #[cfg(feature = "loop_duration")]
    #[prop_or_default]
    pub duration: Option<Signal<crate::duration::DurationInfo>>,
}

impl<C: KeyedCollection> PartialEq for Props<C> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<C: KeyedCollection> Eq for Props<C> {}

pub struct For<T: Clone + PartialEq + Default + 'static, C: KeyedCollection> {
    collection: Signal<C>,
    values:     Vec<Html>,
    t:          PhantomData<T>,
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
        let collection = ctx.props().values.clone();
        let values = Self::make_values(&ctx.props().children, collection.clone());

        {
            let scope = ctx.link().clone();
            let values = collection.clone();

            values.runtime().create_effect(move || {
                values.with(|_| ());
                scope.send_message(Msg::Update);
            });
        }

        Self {
            values,
            collection,
            t: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                self.values = Self::make_values(&ctx.props().children, self.collection.clone());

                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().values != old_props.values {
            ctx.props().values.link_to(&self.collection);
        }

        if ctx.props().children != old_props.children {
            Component::update(self, ctx, Msg::Update)
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        #[cfg(feature = "loop_duration")]
        if let Some(duration) = ctx.props().duration.as_ref().cloned() {
            duration.with(|d| d.begin());
        }

        html! {
            <>
                {self.values.clone()}
            </>
        }
    }

    #[cfg(feature = "loop_duration")]
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(duration) = ctx.props().duration.as_ref().cloned() {
            duration.update(|d| {
                d.end();
            })
        }
    }
}
