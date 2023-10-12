use core::any::Any;
use core::hash::Hash;

use anyhow::Result;

pub trait Store<T> {
    type Key;

    async fn entry(&self, key: Self::Key) -> Result<impl Entry<T>>;
}

pub trait Entry<T> {
    async fn is_empty(&self) -> Result<bool>;

    async fn set(self, val: T) -> Result<bool>;
    async fn get(self) -> Result<Option<T>>;
}

pub struct InMemoryStore<K>(dashmap::DashMap<K, Box<dyn Any>>);

impl<K: Eq + Hash> InMemoryStore<K> {
    pub fn new() -> Self { Self(dashmap::DashMap::new()) }
}

impl<K: Eq + Hash> Default for InMemoryStore<K> {
    fn default() -> Self { Self::new() }
}

impl<K: Eq + Hash, T: Any> Store<T> for InMemoryStore<K> {
    type Key = K;

    async fn entry(&self, key: Self::Key) -> Result<impl Entry<T>> {
        Ok(InMemoryEntry(self.0.entry(key)))
    }
}

pub struct InMemoryEntry<'a, K, V>(dashmap::mapref::entry::Entry<'a, K, V>);

impl<K: Eq + Hash, T: Any> Entry<T> for InMemoryEntry<'_, K, Box<dyn Any>> {
    async fn is_empty(&self) -> Result<bool> {
        use dashmap::mapref::entry::Entry;

        match self.0 {
            Entry::Occupied(_) => Ok(true),
            Entry::Vacant(_) => Ok(false),
        }
    }

    async fn set(self, val: T) -> Result<bool> {
        use dashmap::mapref::entry::Entry;

        let is_inserted = match self.0 {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(Box::new(val));
                true
            },
        };

        Ok(is_inserted)
    }

    async fn get(self) -> Result<Option<T>> {
        use dashmap::mapref::entry::Entry;

        match self.0 {
            Entry::Vacant(_) => Ok(None),
            Entry::Occupied(e) => match e.get().downcast_ref::<T>() {
                Some(_) => Ok(Some(*e.remove().downcast::<T>().unwrap())),
                None => Err(anyhow::anyhow!("unmatched type")),
            },
        }
    }
}
