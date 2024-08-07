use std::collections::{HashMap, BTreeMap};

pub trait KeyedCollection: 'static {
    type Value;

    fn keyed_get(&self, key: &str) -> Option<&Self::Value>;

    fn iter_keys(&self) -> impl Iterator<Item = String>;
}

impl<V: 'static> KeyedCollection for HashMap<String, V> {
    type Value = V;

    fn keyed_get(&self, key: &str) -> Option<&V> {
        HashMap::<String, V>::get(self, key)
    }

    fn iter_keys(&self) -> impl Iterator<Item = String> {
        HashMap::<String, V>::keys(self).cloned()
    }
}

impl<V: 'static> KeyedCollection for BTreeMap<String, V> {
    type Value = V;

    fn keyed_get(&self, key: &str) -> Option<&V> {
        BTreeMap::<String, V>::get(self, key)
    }

    fn iter_keys(&self) -> impl Iterator<Item = String> {
        BTreeMap::<String, V>::keys(self).cloned()
    }
}

impl<V: 'static> KeyedCollection for Vec<V> {
    type Value = V;

    fn keyed_get(&self, key: &str) -> Option<&V> {
        let idx: usize = key.parse().ok()?;

        self.get(idx)
    }

    fn iter_keys(&self) -> impl Iterator<Item = String> {
        (0..self.len()).map(|i| i.to_string())
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_keyed_collection_for_hash_map() {
        let mut map = HashMap::new();
        let keys = vec![String::from("a"), String::from("b")]
            .into_iter()
            .collect::<HashSet<_>>();

        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        assert_eq!(map.keyed_get("a"), Some(&1));
        assert_eq!(map.keyed_get("b"), Some(&2));
        assert_eq!(map.keyed_get("c"), None);
        assert_eq!(map.iter_keys().collect::<HashSet<_>>(), keys);
    }

    #[test]
    fn test_keyed_collection_for_btree_map() {
        let mut map = BTreeMap::new();
        let keys = vec![String::from("a"), String::from("b")]
            .into_iter()
            .collect::<HashSet<_>>();

        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        assert_eq!(map.keyed_get("a"), Some(&1));
        assert_eq!(map.keyed_get("b"), Some(&2));
        assert_eq!(map.keyed_get("c"), None);
        assert_eq!(map.iter_keys().collect::<HashSet<_>>(), keys);
    }

    #[test]
    fn test_keyed_collection_for_vec() {
        let vec = vec![1, 2, 3];
        let keys = vec![String::from("0"), String::from("1"), String::from("2")]
            .into_iter()
            .collect::<HashSet<_>>();

        assert_eq!(vec.keyed_get("0"), Some(&1));
        assert_eq!(vec.keyed_get("1"), Some(&2));
        assert_eq!(vec.keyed_get("2"), Some(&3));
        assert_eq!(vec.keyed_get("3"), None);
        assert_eq!(vec.iter_keys().collect::<HashSet<_>>(), keys);
    }
}
// no-coverage:stop
