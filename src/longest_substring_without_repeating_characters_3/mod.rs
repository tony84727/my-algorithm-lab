pub mod sliding_window_slow;

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::sliding_window_slow::Solution as SlidingWindowSlow;

    #[test_case("abcabcbb" => 3; "example 1")]
    #[test_case("bbbbb" => 1; "example 2")]
    #[test_case("abba" => 2; "case 1")]
    #[test_case("dvdf" => 3; "case 2")]
    fn test_sliding_window_slow_solution(input: &str) -> i32 {
        SlidingWindowSlow::length_of_longest_substring(input.to_string())
    }
}
