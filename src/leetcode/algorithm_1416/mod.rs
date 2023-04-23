pub mod dp;
pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1000", 10000 => 1; "example 1")]
    #[test_case("1000", 10 => 0; "example 2")]
    #[test_case("1317", 2000 => 8; "example 3")]
    fn test_recursive(s: &str, k: i32) -> i32 {
        recursive::Solution::number_of_arrays(s.to_owned(), k)
    }

    #[test_case("1000", 10000 => 1; "example 1")]
    #[test_case("1000", 10 => 0; "example 2")]
    #[test_case("1317", 2000 => 8; "example 3")]
    fn test_dp(s: &str, k: i32) -> i32 {
        dp::Solution::number_of_arrays(s.to_owned(), k)
    }
}
