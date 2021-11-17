pub mod recursive;
pub mod table_memoized;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3,7 => 28; "example 1")]
    #[test_case(3,2 => 3; "example 2")]
    #[test_case(7,3 => 28; "example 3")]
    #[test_case(3,3 => 6; "example 4")]
    fn test_brute_force(m: i32, n: i32) -> i32 {
        recursive::Solution::unique_paths(m, n)
    }

    #[test_case(3,7 => 28; "example 1")]
    #[test_case(3,2 => 3; "example 2")]
    #[test_case(7,3 => 28; "example 3")]
    #[test_case(3,3 => 6; "example 4")]
    fn test_table_memoized(m: i32, n: i32) -> i32 {
        table_memoized::Solution::unique_paths(m, n)
    }
}
