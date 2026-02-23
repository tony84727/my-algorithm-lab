use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let goal = 1 << k;
        if s.len() - k + 1 < goal {
            return false;
        }
        let mut codes = HashSet::with_capacity(goal);
        let mut current = s
            .chars()
            .take(k)
            .enumerate()
            .map(|(i, c)| if c == '1' { 1 << i } else { 0 })
            .sum::<i32>();
        for c in s.chars().skip(k) {
            codes.insert(current);
            current >>= 1;
            current += if c == '1' { 1 << (k - 1) } else { 0 };
        }
        codes.insert(current);
        codes.len() >= goal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("00110110", 2 => true; "example 1")]
    #[test_case("0110", 1 => true; "example 2")]
    #[test_case("0110", 2 => false; "example 3")]
    #[test_case("00110", 2 => true; "case 1")]
    #[test_case("01100", 2 => true; "case 2")]
    fn test_solution(s: &str, k: i32) -> bool {
        Solution::has_all_codes(String::from(s), k)
    }
}
