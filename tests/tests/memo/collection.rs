use super::value::Value;
use yew_reactor::signal::KeyedCollection;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub(super) struct Collection {
    map: HashMap<String, Value>
}

impl Collection {
    pub(super) fn put(&mut self, key: &str, value: Value) {
        self.map.insert(key.to_string(), value);
    }
}

impl KeyedCollection for Collection {
    type Value = Value;

    fn keyed_get(&self, key: &str) -> Option<&Self::Value> {
        self.map.get(key)
    }

    fn iter_keys(&self) -> impl Iterator<Item=String> {
        self.map.keys().map(|s| s.clone())
    }
}
