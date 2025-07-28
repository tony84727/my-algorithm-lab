pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let maximum = nums.iter().fold(0, |a, b| a | b);
        Self::count_subset(&nums, maximum)
    }

    fn count_subset(nums: &[i32], maximum: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums[0];
        let count = if !n & maximum == 0 { 1 } else { 0 };
        count
            + Self::count_subset(&nums[1..], maximum & !n)
            + Self::count_subset(&nums[1..], maximum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,1] => 2; "example 1")]
    #[test_case(vec![2,2,2] => 7; "example 2")]
    #[test_case(vec![3,2,1,5] => 6; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::count_max_or_subsets(nums)
    }
}
