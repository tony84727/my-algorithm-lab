pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .collect();
        for (i, c) in chars.iter().enumerate().take(chars.len() / 2) {
            let counterpart = chars.len() - 1 - i;
            let counterpart = &chars[counterpart];
            if c != counterpart {
                return false;
            }
        }
        true
    }
}
