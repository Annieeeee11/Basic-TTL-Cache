use std::collections::HashMap;
use std::time::Duration;

use crate::entry::Entry;
use crate::time;

pub struct Cache<K, V> {
    pub store: HashMap<K, Entry<V>>,
    pub hits: u64,
    pub misses: u64,
}

impl<K: std::hash::Hash + Eq + Clone, V> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            store: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V, ttl: Duration) {
        let expires_at = time::now() + ttl;
        let entry = Entry {
            value,
            expires_at,
        };
        self.store.insert(key, entry);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let now = time::now();

        let expired = match self.store.get(key) {
            Some(entry) => entry.expires_at <= now,
            None => {
                self.misses += 1;
                return None
            },
        };

        if expired {
            self.misses += 1;
            self.store.remove(key);
            None
        } else {
            self.hits += 1;
            self.store.get(key).map(|entry| &entry.value)
        }
    }

    pub fn hits(&self) -> u64 {
        self.hits
    }

    pub fn misses(&self) -> u64 {
        self.misses
    }

    pub fn cleanup(&mut self) {
        let now = time::now();
        let expired_keys: Vec<K> = self.store
            .iter()
            .filter(|(_, entry)| entry.expires_at <= now)
            .map(|(key, _)| key.clone())
            .collect();
        for key in expired_keys {
            self.store.remove(&key);
        }
    }
}
