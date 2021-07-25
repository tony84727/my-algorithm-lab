use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 0 {
            return 0;
        }
        let mut start = 0;
        let mut end = 1;
        let mut max = 1;
        let len = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut characters = HashSet::new();
        characters.insert(s[0]);
        while start < len {
            if end >= s.len() {
                break;
            }
            let new_char = s[end];
            if characters.contains(&new_char) {
                for i in start..end {
                    characters.remove(&s[i]);
                    if s[i] == new_char {
                        start = i + 1;
                        break;
                    }
                }
            } else {
                characters.insert(new_char);
                end += 1;
                let size = end - start;
                if size > max {
                    max = size;
                }
            }
        }
        max as i32
    }
}
