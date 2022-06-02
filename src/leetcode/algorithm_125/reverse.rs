pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|&c| ('a'..='z').contains(&c) || ('0'..='9').contains(&c))
            .collect();
        let reverse = {
            let mut c = chars.clone();
            c.reverse();
            c
        };
        reverse == chars
    }
}