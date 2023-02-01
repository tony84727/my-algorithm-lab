pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .collect();
        let reverse = {
            let mut c = chars.clone();
            c.reverse();
            c
        };
        reverse == chars
    }
}
