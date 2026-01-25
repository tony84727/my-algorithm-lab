pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        nums.sort_unstable();
        let n = nums.len();
        let mut min = nums[n - 1] - nums[0];
        let k = k as usize;
        for (i, start) in nums.iter().enumerate().take(n.saturating_sub(k - 1)) {
            min = min.min(nums[i + k - 1] - start);
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![90], 1 => 0; "example 1")]
    #[test_case(vec![9,4,1,7], 2 => 2; "example 2")]
    #[test_case(vec![87063,61094,44530,21297,95857,93551,9918], 6 => 74560; "case 1")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i32 {
        Solution::minimum_difference(nums, k)
    }
}
