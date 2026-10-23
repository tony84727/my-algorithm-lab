pub struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let m = nums.len();
        let n = m / 2;
        nums.sort_unstable();
        let mut max = 0;
        for (i, &a) in nums.iter().enumerate().take(n) {
            max = max.max(a + nums[m - 1 - i]);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,5,2,3] => 7; "example 1")]
    #[test_case(vec![3,5,4,2,4,6] => 8; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::min_pair_sum(nums)
    }
}
