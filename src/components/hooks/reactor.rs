use super::super::reactor::ReactorDataContext;
use crate::signal::Runtime;
use yew::{hook, use_context};
use std::sync::Arc;

#[hook]
pub fn use_reactor() -> Arc<Runtime> {
    match use_context::<ReactorDataContext>().map(|ctx| ctx.runtime()) {
        Some(rt) => rt,
        None => Runtime::new(),
    }
}
