use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<i32, Vec<String>> = HashMap::new();
        for s in strs {
            let mut hash = 0;
            for c in s.chars() {
                hash |= 1 << (c as u8) - 97;
            }
            map.entry(hash).or_default().push(s)
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}
