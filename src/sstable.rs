use std::collections::BTreeMap;

pub struct SSTable {
    entries: BTreeMap<String, Option<String>>,
    bloom_filter: u64,
}

impl SSTable {
    pub fn get(&self, key: &str) -> Option<Option<String>> {
        let hash = hash_key(key);
        if (self.bloom_filter & hash) != hash {
            return None; // Definitely not present
        }
        self.entries.get(key).cloned()
    }

    pub fn scan(&self, start: &str, end: &str) -> Vec<(String, Option<String>)> {
        self.entries
            .range(start.to_string()..=end.to_string())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

pub struct SSTableBuilder {
    entries: BTreeMap<String, Option<String>>,
    bloom_filter: u64,
}

impl SSTableBuilder {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            bloom_filter: 0,
        }
    }

    pub fn add(&mut self, key: String, value: Option<String>) {
        self.bloom_filter |= hash_key(&key);
        self.entries.insert(key, value);
    }

    pub fn build(self) -> SSTable {
        SSTable {
            entries: self.entries,
            bloom_filter: self.bloom_filter,
        }
    }
}

fn hash_key(key: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in key.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    1 << (h % 64)
}
