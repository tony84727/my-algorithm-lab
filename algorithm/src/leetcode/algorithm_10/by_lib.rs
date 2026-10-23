use std::str::FromStr;

pub struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let pattern = regex::Regex::from_str(&format!("^{p}$")).unwrap();
        pattern.is_match(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aa", "a" => false; "example 1")]
    #[test_case("aa", "a*" => true; "example 2")]
    #[test_case("ab", ".*" => true; "example 3")]
    fn test_solution(s: &str, p: &str) -> bool {
        Solution::is_match(String::from(s), String::from(p))
    }
}
