use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;
use yew::prelude::*;

pub trait KeyedItem {
    type Key: Eq + Hash;
    fn key(&self) -> Self::Key;
}

#[derive(Debug, PartialEq)]
pub struct ReducibleHashMap<K: Eq + Hash + Clone, V: KeyedItem<Key = K> + Clone>(pub HashMap<K, V>);

impl<K: Eq + Hash + Clone, V: KeyedItem<Key = K> + Clone> Default for ReducibleHashMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub enum ReducibleHashMapAction<V> {
    Single(V),
    Batch(Vec<V>)
}

impl<K: Eq + Hash + Clone, V: KeyedItem<Key = K> + Clone> Reducible for ReducibleHashMap<K, V> {
    type Action = ReducibleHashMapAction<V>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut map = self.0.clone();
        match action {
            ReducibleHashMapAction::Single(item) => {
                map.insert(item.key(), item);
            },
            ReducibleHashMapAction::Batch(items) => {
                for item in items {
                    map.insert(item.key(), item);
                }
            },
        }
        ReducibleHashMap(map).into()
    }
}

pub type HashMapContext<K, V> = UseReducerHandle<ReducibleHashMap<K, V>>;