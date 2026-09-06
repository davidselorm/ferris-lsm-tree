pub struct SSTableEntry {
    pub key: Vec<u8>,
    pub val: Vec<u8>,
}

pub struct SSTable {
    pub entries: Vec<SSTableEntry>,
}

impl SSTable {
    pub fn from_entries(entries: Vec<SSTableEntry>) -> Self {
        Self { entries }
    }
}
