use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        puzzles
            .into_iter()
            .map(|puzzle| {
                let puzzle_characters = {
                    let mut s = HashSet::new();
                    puzzle.chars().for_each(|c| {
                        s.insert(c);
                    });
                    s
                };
                let puzzle_letters: Vec<char> = puzzle.chars().collect();
                words
                    .iter()
                    .filter(|w| {
                        let characters = w.chars();
                        let character_set = {
                            let mut s = HashSet::new();
                            characters.for_each(|c| {
                                s.insert(c);
                            });
                            s
                        };
                        character_set.contains(&puzzle_letters[0])
                            && puzzle_characters.is_superset(&character_set)
                    })
                    .count() as i32
            })
            .collect()
    }
}
