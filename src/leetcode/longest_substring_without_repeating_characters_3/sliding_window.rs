use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 0 {
            return 0;
        }
        let s = s.chars().collect::<Vec<char>>();
        let mut max = 0;
        let mut used = HashSet::new();
        let mut end = 0;
        for i in 0..s.len() {
            while end < s.len() && !used.contains(&s[end]) {
                used.insert(s[end]);
                end += 1;
            }
            let size = end - i;
            if size > max {
                max = size;
            }
            used.remove(&s[i]);
        }
        max as i32
    }
}
