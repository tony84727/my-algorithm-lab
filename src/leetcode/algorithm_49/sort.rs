use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut buckets: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        strs.into_iter().for_each(|anagram| {
            let mut key = anagram.chars().collect::<Vec<char>>();
            key.sort_unstable();
            buckets.entry(key).or_default().push(anagram);
        });
        buckets.into_iter().map(|(_, value)| value).collect()
    }
}
