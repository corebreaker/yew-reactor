use super::use_reactor;
use crate::signal::Signal;
use yew::hook;

#[hook]
pub fn use_signal<T: 'static>(v: T) -> Signal<T> {
    use_reactor().create_signal(v)
}
