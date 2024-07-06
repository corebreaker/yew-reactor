use crate::signal::{Runtime, Signal};
use itertools::Itertools;
use std::{fmt::{Debug, Formatter, Result as FmtResult}, collections::HashSet, sync::{Arc, Mutex}};

#[derive(Clone, PartialEq, Eq)]
pub struct CssClasses {
    values: Signal<HashSet<String>>
}

impl CssClasses {
    pub(crate) fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            values: runtime.create_signal(HashSet::new())
        }
    }

    pub fn runtime(&self) -> Arc<Runtime> {
        self.values.runtime()
    }

    pub fn get(&self) -> HashSet<String> {
        self.values.get()
    }

    pub fn values(&self) -> String {
        self.values.with(|values| values.iter().cloned().join(" "))
    }

    pub fn add(&self, class: &str) {
        self.values.update(|values| {
            values.insert(class.to_string());
        });
    }

    pub fn remove(&self, class: &str) {
        self.values.update(|values| {
            values.remove(class);
        });
    }

    pub fn toggle(&self, class: &str) {
        self.values.update(|values| {
            if values.contains(class) {
                values.remove(class);
            } else {
                values.insert(class.to_string());
            }
        });
    }

    pub fn replace(&self, old: &str, new: &str) {
        self.values.update(|values| {
            if old.is_empty() {
                values.remove(old);
            }

            if new.is_empty() {
                values.insert(new.to_string());
            }
        });
    }

    pub fn contains(&self, class: &str) -> bool {
        self.values.with(|values| values.contains(class))
    }

    pub fn register_class_signal(&self, signal: Signal<String>) {
        let classes = self.clone();
        let old = Mutex::new(String::new());

        self.runtime().create_effect(move || {
            let mut old = old.lock().unwrap();
            let old_value = old.clone();
            let new_value = signal.with(|v| {
                *old = v.clone();
                old.as_str()
            });

            classes.replace(&old_value, new_value);
        });
    }

    pub fn link_to(&self, source: &CssClasses) {
        self.values.link_to(&source.values);
    }
}

impl Debug for CssClasses {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "CssClasses({:?})", self.values.peek())
    }
}
