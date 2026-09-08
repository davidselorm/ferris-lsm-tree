pub mod sstable;

use std::collections::BTreeMap;
use std::sync::{RwLock, Arc};
use sstable::{SSTable, SSTableBuilder};

pub struct LsmTree {
    memtable: Arc<RwLock<BTreeMap<String, Option<String>>>>,
    memtable_max_size: usize,
    sstables: Arc<RwLock<Vec<SSTable>>>,
}

impl LsmTree {
    pub fn new(memtable_max_size: usize) -> Self {
        Self {
            memtable: Arc::new(RwLock::new(BTreeMap::new())),
            memtable_max_size,
            sstables: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        let mut mem = self.memtable.write().unwrap();
        mem.insert(key.to_string(), Some(value.to_string()));

        if mem.len() >= self.memtable_max_size {
            let mut builder = SSTableBuilder::new();
            for (k, v) in mem.iter() {
                builder.add(k.clone(), v.clone());
            }
            let sstable = builder.build();
            self.sstables.write().unwrap().push(sstable);
            mem.clear();
        }
    }

    pub fn delete(&self, key: &str) {
        let mut mem = self.memtable.write().unwrap();
        // Insert tombstone
        mem.insert(key.to_string(), None);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        // 1. Check MemTable
        {
            let mem = self.memtable.read().unwrap();
            if let Some(entry) = mem.get(key) {
                return entry.clone();
            }
        }

        // 2. Check SSTables from newest to oldest
        let tables = self.sstables.read().unwrap();
        for table in tables.iter().rev() {
            if let Some(res) = table.get(key) {
                return res;
            }
        }

        None
    }

    pub fn scan(&self, start: &str, end: &str) -> Vec<(String, String)> {
        let mut merged: BTreeMap<String, Option<String>> = BTreeMap::new();

        // Older SSTables first
        let tables = self.sstables.read().unwrap();
        for table in tables.iter() {
            for (k, v) in table.scan(start, end) {
                merged.insert(k, v);
            }
        }

        // MemTable overrides
        let mem = self.memtable.read().unwrap();
        for (k, v) in mem.range(start.to_string()..=end.to_string()) {
            merged.insert(k.clone(), v.clone());
        }

        merged.into_iter()
            .filter_map(|(k, v)| v.map(|val| (k, val)))
            .collect()
    }
}
