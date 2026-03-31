use std::collections::HashMap;

pub(crate) struct Entry {
    value: String,
    expires_at: Option<u64>,
}

impl Entry {
    pub(crate) fn new(value: String, expires_at: Option<u64>) -> Entry {
        Entry { value, expires_at }
    }

    pub(crate) fn is_expired(&self, now: u64) -> bool {
        match self.expires_at {
            Some(exp) => now >= exp,
            None => false,
        }
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

pub(crate) struct Store {
    map: HashMap<String, Entry>,
}

impl Store {
    pub(crate) fn new() -> Store {
        Store {
            map: HashMap::new(),
        }
    }

    pub(crate) fn set(
        &mut self,
        key: String,
        value: String,
        expires_at: Option<u64>,
    ) -> Option<Entry> {
        let entry = Entry::new(value, expires_at);

        self.map.insert(key, entry)
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Entry> {
        self.map.get(key)
    }

    pub(crate) fn delete(&mut self, key: &str) -> bool {
        self.map.remove(key).is_some()
    }
}
