pub struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, c| acc ^ c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,2,3,3,4,4,8,8] => 2; "example 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::single_non_duplicate(nums)
    }
}
