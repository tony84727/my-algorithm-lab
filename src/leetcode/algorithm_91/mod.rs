pub mod first;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("12" => 2; "example 1")]
    #[test_case("06" => 0; "case 1")]
    #[test_case("0" => 0; "case 2")]
    #[test_case("10" => 1; "case 3")]
    fn test_first_solution(s: &str) -> i32 {
        first::Solution::num_decodings(s.to_string())
    }
}
