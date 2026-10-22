pub struct Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        fn to_bitmap(input: &str) -> i32 {
            let characters = input.chars();
            characters.fold(0, |acc, c| acc | 1 << ((c as u8) - 97))
        }
        let hashed: Vec<i32> = words.into_iter().map(|w| to_bitmap(&w)).collect();
        puzzles
            .into_iter()
            .map(|puzzle| {
                let mask = to_bitmap(&puzzle);
                let must = to_bitmap(&puzzle[0..1]);
                hashed
                    .iter()
                    .filter(|h| *h & must > 0 && !mask & *h == 0)
                    .count() as i32
            })
            .collect()
    }
}
