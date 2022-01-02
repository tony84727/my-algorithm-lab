use std::collections::HashMap;
use std::num::Wrapping;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        const PRIMES: [i32; 26] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
            97, 101, 103,
        ];
        fn hash_string(input: &str) -> i32 {
            input
                .chars()
                .fold(Wrapping(1_i32), |carry, c| {
                    carry * Wrapping(PRIMES[(c as usize - 97)])
                })
                .0
        }
        let mut map: HashMap<i32, Vec<String>> = HashMap::new();
        for s in strs {
            let hash = hash_string(&s);
            map.entry(hash).or_default().push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}
