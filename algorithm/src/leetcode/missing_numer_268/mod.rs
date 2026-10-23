pub mod fold;
pub mod fold_fold;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,0,1] => 2; "example 1")]
    #[test_case(vec![0,1] => 2)]
    fn test_fold_fold_solution(nums: Vec<i32>) -> i32 {
        fold_fold::Solution::missing_number(nums)
    }

    #[test_case(vec![3,0,1] => 2; "example 1")]
    #[test_case(vec![0,1] => 2)]
    fn test_fold_solution(nums: Vec<i32>) -> i32 {
        fold::Solution::missing_number(nums)
    }
}
