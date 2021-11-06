pub mod bysort;
pub mod byxor;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,2,1] => 1; "example 1")]
    #[test_case(vec![4,1,2,1,2] => 4; "example 2")]
    fn test_bysort_solution(nums: Vec<i32>) -> i32 {
        super::bysort::Solution::single_number(nums)
    }

    #[test_case(vec![2,2,1] => 1; "example 1")]
    #[test_case(vec![4,1,2,1,2] => 4; "example 2")]
    fn test_byxor_solution(nums: Vec<i32>) -> i32 {
        super::bysort::Solution::single_number(nums)
    }
}
