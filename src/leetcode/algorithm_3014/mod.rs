use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freqency = vec![0; 26];
        let bytes = word.as_bytes();
        for c in bytes.iter() {
            freqency[(*c - b'a') as usize] += 1;
        }
        let mut sorted: Vec<(usize, usize)> = freqency.into_iter().enumerate().collect();
        sorted.sort_unstable_by_key(|(_, count)| Reverse(*count));
        let mut count = 0;
        let mut level = 1;
        let mut characters = 0;
        for (_, f) in sorted.into_iter() {
            count += f * level;
            characters += 1;
            if characters == 8 {
                characters = 0;
                level += 1;
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abcde" => 5; "example 1")]
    #[test_case("xycdefghij" => 12; "example 2")]
    fn test_solution(word: &str) -> i32 {
        Solution::minimum_pushes(word.to_string())
    }
}
