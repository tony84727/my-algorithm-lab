pub mod brute;
pub mod fixed_length_sliding_window;
pub mod sliding_window;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ab", "eidbaooo" => true; "example 1")]
    #[test_case("ab", "eidboaoo" => false; "example 2")]
    #[test_case("adc", "dcda" => true; "case 1")]
    fn test_brute_solution(s1: &str, s2: &str) -> bool {
        brute::Solution::check_inclusion(s1.to_owned(), s2.to_owned())
    }

    #[test_case("ab", "eidbaooo" => true; "example 1")]
    #[test_case("ab", "eidboaoo" => false; "example 2")]
    #[test_case("adc", "dcda" => true; "case 1")]
    fn test_sliding_window_solution(s1: &str, s2: &str) -> bool {
        sliding_window::Solution::check_inclusion(s1.to_owned(), s2.to_owned())
    }

    #[test_case("ab", "eidbaooo" => true; "example 1")]
    #[test_case("ab", "eidboaoo" => false; "example 2")]
    #[test_case("adc", "dcda" => true; "case 1")]
    fn test_fixed_length_sliding_window(s1: &str, s2: &str) -> bool {
        fixed_length_sliding_window::Solution::check_inclusion(s1.to_owned(), s2.to_owned())
    }
}
