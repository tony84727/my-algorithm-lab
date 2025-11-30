pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().sum::<i32>() % k
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,9,7], 5 => 4; "example 1")]
    #[test_case(vec![4,1,3], 4 => 0; "example 2")]
    #[test_case(vec![3,2], 6 => 5; "example 3")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i32 {
        Solution::min_operations(nums, k)
    }
}
