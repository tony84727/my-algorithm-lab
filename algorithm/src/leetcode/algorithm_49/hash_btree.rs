//! Because BTreeMap is hashable. Store letter apperance count in a BTreeMap. And use BTreeMap as the hash key to group anagrams.
//!
//! Steps:
//! 1. For each word, construct a BTree to record letter apperance count
//! 2. Hash the BTree to get a hash value. Use the hash value to construct a HashMap. The buckets are the anagrams with the same letters
use std::collections::{BTreeMap, HashMap};

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(
                HashMap::<BTreeMap<char, i32>, Vec<String>>::new(),
                |mut carry, current| {
                    carry
                        .entry(current.chars().fold(BTreeMap::new(), |mut carry, current| {
                            *carry.entry(current).or_default() += 1;
                            carry
                        }))
                        .or_default()
                        .push(current);
                    carry
                },
            )
            .into_values()
            .collect()
    }
}
