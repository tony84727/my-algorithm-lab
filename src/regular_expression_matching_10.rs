pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aa", "a", true; "example_1")]
    #[test_case("aa", "a*", true; "example_2")]
    #[test_case("ab", ".*", true; "example_3")]
    #[test_case("mississippi", "mis*is*p*.", false; "example_4")]
    fn test_is_match(input: &str, pattern: &str, expected: bool) {
        assert_eq!(expected, Solution::is_match(input.to_string(), pattern.to_string()))
    }
}