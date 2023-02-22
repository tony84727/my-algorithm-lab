pub mod binary_search;
pub mod xor;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,2,3,3,4,4,8,8] => 2; "example 1")]
    #[test_case(vec![3,3,7,7,10,11,11] => 10; "example 2")]
    #[test_case(vec![1,1,2,2,3,4,4,8,8] => 3; "case 1")]
    #[test_case(vec![1,1,2,2,3,3,4,8,8] => 4; "case 2")]
    #[test_case(vec![1] => 1; "case 3")]
    #[test_case(vec![1,1,2] => 2; "case 4")]
    fn test_xor_solution(nums: Vec<i32>) -> i32 {
        xor::Solution::single_non_duplicate(nums)
    }

    #[test_case(vec![1,1,2,3,3,4,4,8,8] => 2; "example 1")]
    #[test_case(vec![3,3,7,7,10,11,11] => 10; "example 2")]
    #[test_case(vec![1,1,2,2,3,4,4,8,8] => 3; "case 1")]
    #[test_case(vec![1,1,2,2,3,3,4,8,8] => 4; "case 2")]
    #[test_case(vec![1] => 1; "case 3")]
    #[test_case(vec![1,1,2] => 2; "case 4")]
    fn test_binary_search_solution(nums: Vec<i32>) -> i32 {
        binary_search::Solution::single_non_duplicate(nums)
    }
}
