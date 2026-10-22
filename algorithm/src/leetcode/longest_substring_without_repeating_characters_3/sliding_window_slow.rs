use std::collections::HashSet;
pub struct Solution;

impl Solution {
    // clippy false positive: https://github.com/rust-lang/rust-clippy/issues/6072
    #[allow(clippy::mut_range_bound)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
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
                for (i, c) in s.iter().enumerate().take(end).skip(start) {
                    characters.remove(&s[i]);
                    if c == &new_char {
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
