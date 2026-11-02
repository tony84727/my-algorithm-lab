pub struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let middle_value = nums[n / 2];
        nums.into_iter()
            .map(|x| x.abs_diff(middle_value) as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 3] => 2; "example 1")]
    #[test_case(vec![1, 10, 2, 9] => 16; "example 2")]
    #[test_case(vec![1,0,0,8,6] => 14; "case 1")]
    fn test_min_moves2(nums: Vec<i32>) -> i32 {
        Solution::min_moves2(nums)
    }
}
