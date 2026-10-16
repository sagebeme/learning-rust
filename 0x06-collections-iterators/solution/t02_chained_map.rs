//! Task 2: build a hash map by hand (your ALX hash table, in Rust).
//!
//! `ChainedMap` stores key/value pairs in a fixed number of buckets. A key's bucket is
//! `djb2(key) % number_of_buckets`. Keys that land in the same bucket share it as a list (a
//! "chain"). Rules:
//! - `new(buckets)` makes at least 1 bucket, even if asked for 0
//! - `insert` adds a new key, or replaces the value of an existing key and returns the old value
//! - `get` finds a key's value, `len` counts keys (not buckets)

/// The djb2 string hash, as in your ALX `hash_djb2`. Given.
pub fn djb2(key: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in key.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(u64::from(byte));
    }
    hash
}

pub struct ChainedMap {
    buckets: Vec<Vec<(String, String)>>,
}

impl ChainedMap {
    pub fn new(buckets: usize) -> ChainedMap {
        ChainedMap {
            buckets: vec![Vec::new(); buckets.max(1)],
        }
    }

    fn bucket_index(&self, key: &str) -> usize {
        // The remainder is smaller than the number of buckets, which is a usize, so it fits.
        (djb2(key) % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        let index = self.bucket_index(key);
        let chain = &mut self.buckets[index];
        for (existing_key, existing_value) in chain.iter_mut() {
            if existing_key == key {
                return Some(std::mem::replace(existing_value, value.to_string()));
            }
        }
        chain.push((key.to_string(), value.to_string()));
        None
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.buckets[self.bucket_index(key)]
            .iter()
            .find(|(existing_key, _)| existing_key == key)
            .map(|(_, value)| value.as_str())
    }

    pub fn len(&self) -> usize {
        self.buckets.iter().map(Vec::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
