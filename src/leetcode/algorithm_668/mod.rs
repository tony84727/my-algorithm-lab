pub mod binary_search;
pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3,3,5 => 3; "example 1")]
    #[test_case(2,3,6 => 6; "example 2")]
    fn test_brute_force_solution(m: i32, n: i32, k: i32) -> i32 {
        brute_force::Solution::find_kth_number(m, n, k)
    }

    #[test_case(3,3,5 => 3; "example 1")]
    #[test_case(2,3,6 => 6; "example 2")]
    fn test_binary_search_solution(m: i32, n: i32, k: i32) -> i32 {
        binary_search::Solution::find_kth_number(m, n, k)
    }
}
