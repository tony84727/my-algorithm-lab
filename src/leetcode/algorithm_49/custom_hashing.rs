use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn hash_string(input: &str) -> i32 {
            i32::rotate_left(
                input.chars().fold(0, |carry, c| carry + (c as i32 - 96)),
                input.len() as u32,
            )
        }
        let mut map: HashMap<i32, Vec<String>> = HashMap::new();
        for s in strs {
            let hash = hash_string(&s);
            map.entry(hash).or_default().push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}
