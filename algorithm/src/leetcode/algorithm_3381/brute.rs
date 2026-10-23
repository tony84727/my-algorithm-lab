pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut max = i64::MIN;
        for length in (k..=n).step_by(k) {
            for start in 0..=n - length {
                let mut sum = 0;
                for &n in nums.iter().skip(start).take(length) {
                    let n = n as i64;
                    sum += n;
                }
                max = max.max(sum);
            }
        }
        max
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
