use super::use_reactor;
use crate::signal::Signal;
use yew::{use_state, hook};

#[hook]
pub fn use_signal<T: 'static>(v: T) -> Signal<T> {
    let rt = use_reactor();
    let state = use_state(move || rt.create_signal(v));

    (*state).clone()
}
