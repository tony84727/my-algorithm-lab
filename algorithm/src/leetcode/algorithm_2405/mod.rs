use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut characters = HashSet::new();
        let mut counts = 1;
        for c in s.chars() {
            if characters.contains(&c) {
                counts += 1;
                characters = HashSet::new();
            }
            characters.insert(c);
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abacaba" => 4; "example 1")]
    fn test_solution(s: &str) -> i32 {
        Solution::partition_string(s.to_owned())
    }
}
