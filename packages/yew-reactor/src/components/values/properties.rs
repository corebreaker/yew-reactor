use crate::{signal::Signal, css::CssClasses};
use yew::{AttrValue, Properties};

pub(super) trait ValueProps: Properties {
    fn class(&self) -> Option<&AttrValue>;

    fn class_signal(&self) -> Option<&Signal<String>>;

    fn classes(&self) -> Option<&CssClasses>;

    fn element(&self) -> Option<&AttrValue>;
}
