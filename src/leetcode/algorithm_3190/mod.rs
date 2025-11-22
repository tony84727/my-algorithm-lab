pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|x| x % 3 != 0).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4] => 3; "example 1")]
    #[test_case(vec![3,6,9] => 0; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::minimum_operations(nums)
    }
}
