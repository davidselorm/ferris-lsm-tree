use std::collections::BTreeMap;

pub struct MemTable {
    map: BTreeMap<Vec<u8>, Vec<u8>>,
    size_bytes: usize,
}

impl MemTable {
    pub fn new() -> Self {
        Self { map: BTreeMap::new(), size_bytes: 0 }
    }
    pub fn put(&mut self, key: Vec<u8>, val: Vec<u8>) {
        self.size_bytes += key.len() + val.len();
        self.map.insert(key, val);
    }
    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.map.get(key)
    }
}
