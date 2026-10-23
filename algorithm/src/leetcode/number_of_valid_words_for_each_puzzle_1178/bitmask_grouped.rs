use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        fn to_bitmap(input: &str) -> i32 {
            let characters = input.chars();
            characters.fold(0, |acc, c| acc | 1 << ((c as u8) - 97))
        }
        let hashed = {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for w in words.into_iter() {
                let hash = to_bitmap(&w);
                *map.entry(hash).or_default() += 1;
            }
            map
        };
        puzzles
            .into_iter()
            .map(|puzzle| {
                let mask = to_bitmap(&puzzle);
                let must = to_bitmap(&puzzle[0..1]);
                hashed
                    .iter()
                    .filter(|(k, _)| *k & must > 0 && !mask & *k == 0)
                    .fold(0, |acc, (_, matched_count)| acc + matched_count)
            })
            .collect()
    }
}
