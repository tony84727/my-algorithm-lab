pub mod sliding_window;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ADOBECODEBANC", "ABC", "BANC"; "example 1")]
    #[test_case("a", "aa", "")]
    fn test_sliding_window_solution(s: &str, t: &str, expected: &str) {
        assert_eq!(
            expected.to_string(),
            sliding_window::Solution::min_window(s.to_string(), t.to_string())
        );
    }
}
