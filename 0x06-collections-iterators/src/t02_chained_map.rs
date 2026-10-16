//! Task 2: build a hash map by hand (your ALX hash table, in Rust).
//!
//! `ChainedMap` stores key/value pairs in a fixed number of buckets. A key's bucket is
//! `djb2(key) % number_of_buckets`. Keys that land in the same bucket share it as a list (a
//! "chain"). Rules:
//! - `new(buckets)` makes at least 1 bucket, even if asked for 0
//! - `insert` adds a new key, or replaces the value of an existing key and returns the old value
//! - `get` finds a key's value, `len` counts keys (not buckets)

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

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
        todo!()
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        todo!()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
