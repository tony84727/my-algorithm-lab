pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }

        let mut max_sum = i64::MIN;
        for length in (k..=n).step_by(k) {
            for i in 0..=n - length {
                let current_sum = prefix_sum[i + length] - prefix_sum[i];
                max_sum = max_sum.max(current_sum);
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2], 1 => 3; "example 1")]
    #[test_case(vec![-1,-2,-3,-4,-5], 4 => -10; "example 2")]
    #[test_case(vec![-5,1,2,-3,4], 2 => 4; "example 3")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i64 {
        Solution::max_subarray_sum(nums, k)
    }
}
