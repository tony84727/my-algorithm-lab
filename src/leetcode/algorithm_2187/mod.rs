pub mod binary_search;
pub mod brute;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3], 5 => 3; "example 1")]
    #[test_case(vec![2], 1 => 2; "example 2")]
    fn test_brute_solution(time: Vec<i32>, total_trips: i32) -> i64 {
        brute::Solution::minimum_time(time, total_trips)
    }

    #[test_case(vec![1,2,3], 5 => 3; "example 1")]
    #[test_case(vec![2], 1 => 2; "example 2")]
    fn test_binary_search_solution(time: Vec<i32>, total_trips: i32) -> i64 {
        binary_search::Solution::minimum_time(time, total_trips)
    }
}
