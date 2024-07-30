use super::super::r#loop::{LoopDataContext, LoopVar};
use yew::{hook, use_context};

#[hook]
fn use_loop_var<T: Clone + Default + PartialEq + 'static>() -> LoopVar<T> {
    let ctx: Option<LoopDataContext<T>> = use_context::<LoopDataContext<T>>();

    ctx.as_ref().map(|v| v.get_var()).unwrap_or_default()
}
