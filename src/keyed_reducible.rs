use std::{rc::Rc, collections::HashMap, hash::Hash};

use yew::prelude::*;

pub trait KeyedReducibleItem {
    type Key: Eq + Hash;
    fn key(&self) -> Self::Key;
}

#[derive(Debug, PartialEq)]
pub struct KeyedReducible<K: Eq + Hash + Clone, V: KeyedReducibleItem<Key = K> + Clone> {
    pub map: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: KeyedReducibleItem<Key = K> + Clone> Default for KeyedReducible<K, V> {
    fn default() -> Self {
        Self { map: Default::default() }
    }
}

pub enum KeyedReducibleAction<V> {
    Single(V),
    Batch(Vec<V>)
}

impl<K: Eq + Hash + Clone, V: KeyedReducibleItem<Key = K> + Clone> Reducible for KeyedReducible<K, V> {
    type Action = KeyedReducibleAction<V>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut map = self.map.clone();
        match action {
            KeyedReducibleAction::Single(item) => {
                map.insert(item.key(), item);
            },
            KeyedReducibleAction::Batch(items) => {
                for item in items {
                    map.insert(item.key(), item);
                }
            },
        }
        KeyedReducible { map }.into()
    }
}

pub type UseKeyedReducerHandle<K, V> = UseReducerHandle<KeyedReducible<K, V>>;