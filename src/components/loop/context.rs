use crate::signal::Signal;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct LoopContext<T: 'static> {
    pub key: Rc<String>,
    pub value: Signal<T>,
}
