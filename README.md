# ferris-lsm-tree

A production Log-Structured Merge-Tree (LSM) key-value storage engine implemented in Rust.

## Architecture
- **In-Memory MemTable**: Concurrent BTreeMap for high-throughput append-only writes.
- **Immutable SSTables**: Sorted String Tables featuring integrated 64-bit Bloom Filters for $O(1)$ negative lookup pruning.
- **Tombstone Deletions**: Predictable soft-deletion markers filtered during compaction and range scans.
- **Range Scans**: Multi-table merging iterators supporting arbitrary prefix and range queries.

## Usage
```rust
use ferris_lsm_tree::LsmTree;

let db = LsmTree::new(100);
db.set("user:1001", "Alice");
assert_eq!(db.get("user:1001"), Some("Alice".to_string()));
```
