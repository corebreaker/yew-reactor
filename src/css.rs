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

    pub fn contains(&self, class: &str) -> bool {
        self.values.with(|values| values.contains(class))
    }

    pub fn get(&self) -> HashSet<String> {
        self.values.get()
    }

    pub fn values(&self) -> String {
        self.values.with(|values| values.iter().cloned().join(" "))
    }

    pub fn sorted_values(&self) -> String {
        self.values.with(|values| values.iter().sorted().cloned().join(" "))
    }

    pub fn with_values<T>(&self, f: impl FnOnce(&HashSet<String>) -> T) -> T {
        self.values.with(f)
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
        self.values.update_if(|values| {
            let mut changed = !old.is_empty();
            if changed {
                values.remove(old);
            }

            if !new.is_empty() {
                changed = true;
                values.insert(new.to_string());
            }

            changed
        });
    }

    pub fn register_class_signal(&self, signal: Signal<String>) {
        let classes = self.clone();
        let old = Mutex::new(signal.get());

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

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::tests::create_runtime;

    fn make_css() -> (CssClasses, HashSet<String>) {
        let list = HashSet::from_iter(vec![
            String::from("class1"),
            String::from("class2"),
            String::from("class3"),
        ]);

        let css = CssClasses{
            values: create_runtime().create_signal(list.clone()),
        };

        (css, list)
    }

    #[test]
    fn test_get_css_class() {
        let (classes, list) = make_css();
        let css_list = classes.values()
            .split(" ")
            .map(String::from)
            .collect::<HashSet<_>>();

        assert_eq!(classes.with_values(|values| values.len()), 3, "the CSS classes should have 3 items");
        assert_eq!(classes.get(), list, "getting the CSS classes should return the correct list");
        assert_eq!(css_list, list, "the CSS classes should be listed in a string");
        assert_eq!(classes.sorted_values(), "class1 class2 class3", "CSS values should be listed in a sorted string");
        assert!(classes.contains("class1"), "the CSS class should be contained");
    }

    #[test]
    fn test_add_css_class() {
        let (classes, _) = make_css();

        assert!(classes.contains("class3"));

        classes.add("class3");
        classes.add("class4");

        assert!(classes.contains("class1"), "the CSS class `class1` should be contained");
        assert!(classes.contains("class2"), "the CSS class `class2` should be contained");
        assert!(classes.contains("class3"), "the CSS class `class3` should be contained");
        assert!(classes.contains("class4"), "the CSS class `class4` should be contained");
        assert!(!classes.contains("class6"), "the CSS class `class6` should not be contained");
        assert_eq!(classes.sorted_values(), "class1 class2 class3 class4", "All CSS classes should be added");
    }

    #[test]
    fn test_remove_css_class() {
        let (classes, _) = make_css();

        assert!(classes.contains("class2"), "the CSS class `class2` should be contained before removing");

        classes.remove("class2");

        assert!(!classes.contains("class2"), "the CSS class `class2` should be removed");
        assert_eq!(classes.sorted_values(), "class1 class3", "the CSS class `class2` should not be in the list");
    }

    #[test]
    fn test_toggle_css_class() {
        let (classes, _) = make_css();

        assert!(classes.contains("class2"), "the CSS class `class2` should be contained before toggling");

        classes.toggle("class2");
        classes.toggle("class4");

        assert!(!classes.contains("class2"), "the CSS class `class2` should be removed");
        assert!(classes.contains("class4"), "the CSS class `class4` should be added");
        assert_eq!(
            classes.sorted_values(),
            "class1 class3 class4",
            "the CSS class `class2` should not be in the list and `class4` should be added",
        );
    }

    #[test]
    fn test_replace_css_class() {
        let (classes, _) = make_css();

        assert!(classes.contains("class2"), "the CSS class `class2` should be contained before replacing");

        classes.replace("class2", "class4");

        assert!(!classes.contains("class2"), "the CSS class `class2` should be removed");
        assert!(classes.contains("class4"), "the CSS class `class4` should be added");
        assert_eq!(
            classes.sorted_values(),
            "class1 class3 class4",
            "the CSS class `class2` should not be in the list and `class4` should be added",
        );
    }

    #[test]
    fn test_register_class_signal() {
        let (classes, _) = make_css();
        let signal = classes.runtime().create_signal(String::from("class4"));

        assert!(!classes.contains("class4"), "the CSS class `class4` should not be contained before registering");
        classes.register_class_signal(signal.clone());
        assert!(classes.contains("class4"), "the CSS class `class4` should be added after registering");

        assert_eq!(
            classes.sorted_values(),
            "class1 class2 class3 class4",
            "the CSS class `class4` should be in the list",
        );

        signal.set(String::from("class5"));
        assert_eq!(
            classes.sorted_values(),
            "class1 class2 class3 class5",
            "the CSS class `class4` should be replaced with `class5`",
        );

        signal.set(String::from("class2"));
        assert_eq!(
            classes.sorted_values(),
            "class1 class2 class3",
            "the CSS class `class5` should be removed",
        );

        signal.set(String::from("class9"));
        assert_eq!(
            classes.sorted_values(),
            "class1 class3 class9",
            "the CSS class `class2` should be removed and `class9` should be added",
        );
    }

    #[test]
    fn test_link_css_classes() {
        let (src, _) = make_css();
        let dest = src.runtime().create_css_classes();

        dest.link_to(&src);

        assert_eq!(src.sorted_values(), "class1 class2 class3", "the CSS classes should be linked from the source");
        assert_eq!(dest.sorted_values(), "class1 class2 class3", "the CSS classes should be linked to the source");

        dest.add("class4");

        assert_eq!(src.sorted_values(), "class1 class2 class3 class4", "the source should be updated");
        assert_eq!(dest.sorted_values(), "class1 class2 class3 class4", "the destination should be updated");

        src.remove("class2");

        assert_eq!(src.sorted_values(), "class1 class3 class4", "`class2` should be removed from the source");
        assert_eq!(dest.sorted_values(), "class1 class3 class4", "the removal should be reflected in the destination");
    }
}
// no-coverage:stop
