use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        if text.is_empty() {
            return 0;
        }
        let mut broken = HashSet::new();
        for b in broken_letters.chars() {
            broken.insert(b);
        }
        let mut count = 0;
        let mut failed = false;
        for c in text.chars() {
            if c == ' ' {
                if !failed {
                    count += 1;
                }
                failed = false;
                continue;
            }
            if broken.contains(&c) {
                failed = true;
            }
        }
        if !failed {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hello world", "ad" => 1; "example 1")]
    #[test_case("leet code", "lt" => 1; "example 2")]
    #[test_case("leet code", "e" => 0; "example 3")]
    fn test_solution(text: &str, broken_letters: &str) -> i32 {
        Solution::can_be_typed_words(String::from(text), String::from(broken_letters))
    }
}
