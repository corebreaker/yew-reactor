use super::Record;
use yew_reactor::signal::KeyedCollection;
use uuid::{NoContext, Timestamp, Uuid};
use rand::{seq::IteratorRandom, Rng};
use std::{fmt::{Display, Formatter, Result as FmtResult}, collections::HashMap};

#[derive(Debug, Clone, Default)]
pub struct DataList {
    id: Uuid,
    values: Vec<Record>,
    indexes: HashMap<Uuid, usize>,
}

impl DataList {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v7(Timestamp::now(NoContext)),
            values: vec![],
            indexes: HashMap::new(),
        }
    }

    pub fn get(&self, id: Uuid) -> Option<&Record> {
        self.indexes.get(&id).and_then(|&idx| self.values.get(idx))
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Record> {
        self.indexes.get(&id).and_then(|&idx| self.values.get_mut(idx))
    }

    pub fn contains_id(&self, id: Uuid) -> bool {
        self.indexes.contains_key(&id)
    }

    pub fn contains(&self, record: &Record) -> bool {
        self.contains_id(record.id())
    }

    pub fn values(&self) -> impl Iterator<Item = &Record> {
        self.values.iter()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Record> {
        self.values.iter_mut()
    }

    pub fn ids(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.values.iter().map(|r| r.id())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.indexes.clear();
    }

    pub fn generate(&mut self, count: usize) {
        for _ in 0..count {
            self.insert(Record::generate());
        }
    }

    pub fn insert(&mut self, record: Record) {
        let id = record.id();

        match self.indexes.get(&id) {
            Some(&index) => {
                self.values[index] = record;
            }
            None => {
                self.indexes.insert(id, self.values.len());
                self.values.push(record);
            }
        }
    }

    pub fn remove(&mut self, id: Uuid) -> Option<Record> {
        if let Some(index) = self.indexes.remove(&id) {
            let res = self.values.remove(index);

            for (i, record) in self.values.iter().enumerate().skip(index) {
                self.indexes.insert(record.id(), i);
            }

            return Some(res);
        }

        None
    }

    pub fn random_remove(&mut self, count: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let sz = self.len();
            if sz == 0 {
                break;
            }

            let idx = rng.gen_range(0..sz);
            let id = self.values[idx].id();

            self.remove(id);
        }
    }

    pub fn random_change_occupation(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        let id_list = self.ids().choose_multiple(&mut rng, count);
        let idx_list = id_list.iter().map(|id| self.indexes[id]).collect::<Vec<_>>();

        for i in idx_list {
            self.values[i].change_occupation();
        }
    }

    pub fn random_change_description(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        let id_list = self.ids().choose_multiple(&mut rng, count);
        let idx_list = id_list.iter().map(|id| self.indexes[id]).collect::<Vec<_>>();

        for i in idx_list {
            self.values[i].change_description();
        }
    }

    pub fn rotate(&mut self) {
        self.values.rotate_left(1);
    }
}

impl PartialEq for DataList {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DataList {}

impl Display for DataList {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let sz = self.values.len();
        let plural = if sz > 1 { "s" } else { "" };

        write!(f, "{sz} record{plural}")
    }
}

impl KeyedCollection for DataList {
    type Value = Record;

    fn keyed_get(&self, key: &str) -> Option<&Self::Value> {
        let id = Uuid::parse_str(key).ok()?;
        let idx = self.indexes.get(&id)?;

        self.values.get(*idx)
    }

    fn iter_keys(&self) -> impl Iterator<Item=String> {
        self.ids().map(|id| id.to_string())
    }
}
