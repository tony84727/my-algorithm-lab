pub struct Solution;

impl Solution {
    /**
     * Constrains
     * 1. 3 <= nums.length <= 104
     * 2. -1000 <= nums[i] <= 1000
     */
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let answer = nums[0] * nums[1] * nums.last().unwrap();
        answer.max(nums.into_iter().rev().take(3).product())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 3] => 6; "example 1")]
    #[test_case(vec![1, 2, 3, 4] => 24; "example 2")]
    #[test_case(vec![-1, -2, -3] => -6; "example 3")]
    #[test_case(vec![-2,-3,1,2,4] => 24; "case 1")]
    fn examples(nums: Vec<i32>) -> i32 {
        Solution::maximum_product(nums)
    }
}
