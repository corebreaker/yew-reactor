use super::ReactorDataContext;
use crate::{signal::Runtime, defer::DeferRunner, spawner::SpawnGenerator};
use yew::{context::ContextProvider, Children, Component, Context, Html, Properties, ToHtml, html};
use std::sync::Arc;

#[derive(Properties, Default)]
pub struct Props {
    #[prop_or_default]
    pub(crate) with_defer_runner: Option<Arc<dyn DeferRunner>>,

    #[prop_or_default]
    pub(crate) with_spawn_generator: Option<Arc<dyn SpawnGenerator>>,

    pub(crate) children: Children,
}

impl Eq for Props {}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        if self.children != other.children {
            return false;
        }

        let eq_runner = match (&self.with_defer_runner, &other.with_defer_runner) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };

        let eq_generator = match (&self.with_spawn_generator, &other.with_spawn_generator) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };

        eq_runner && eq_generator
    }
}

pub struct Reactor {
    rt: Arc<Runtime>,
}

impl Reactor {
    pub fn runtime(&self) -> Arc<Runtime> {
        Arc::clone(&self.rt)
    }
}

impl Component for Reactor {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut rt = Runtime::new();

        if let Some(runner) = ctx.props().with_defer_runner.as_ref() {
            rt = rt.with_defer_runner(Arc::clone(runner));
        }

        if let Some(generator) = ctx.props().with_spawn_generator.as_ref() {
            rt = rt.with_spawn_generator(Arc::clone(generator));
        }

        Self {
            rt,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        match ctx.props().with_defer_runner {
            Some(ref runner) => {
                self.rt.defer_manager().set_runner(Arc::clone(runner));
            }
            None => {
                self.rt.defer_manager().reset_runner();
            }
        }

        match ctx.props().with_spawn_generator {
            Some(ref generator) => {
                self.rt.spawner().set_generator(Arc::clone(generator));
            }
            None => {
                self.rt.spawner().reset_generator();
            }
        }

        ctx.props().children != old_props.children
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let children = ctx.props().children.to_html();
        let context = ReactorDataContext::new(Arc::clone(&self.rt));

        html! {
            <ContextProvider<ReactorDataContext> {context}>
                {children}
            </ContextProvider<ReactorDataContext>>
        }
    }
}
