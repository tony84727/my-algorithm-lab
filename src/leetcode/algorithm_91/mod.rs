pub mod first;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("12" => 2; "example 1")]
    #[test_case("06" => 0; "case 1")]
    #[test_case("0" => 0; "case 2")]
    #[test_case("10" => 1; "case 3")]
    #[test_case("2101" => 1; "case 4")]
    #[test_case("1123" => 5; "case 5")]
    #[test_case("112" => 3; "case 6")]
    #[test_case("123123" => 9; "case 7")]
    #[test_case("2611055971756562" => 4; "case 8")]
    #[test_case("26110" => 2; "case 9")]
    #[test_case("2611" => 4; "case 10")]
    #[test_case("12120" => 3; "case 11")]
    fn test_first_solution(s: &str) -> i32 {
        first::Solution::num_decodings(s.to_string())
    }
}
