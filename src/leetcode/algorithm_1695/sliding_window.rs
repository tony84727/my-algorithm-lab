use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut sum = 0;
        let mut numbers = HashSet::new();
        let mut max = 0;
        for &n in nums.iter() {
            if end >= nums.len() {
                break;
            }
            while end < nums.len() && !numbers.contains(&nums[end]) {
                numbers.insert(&nums[end]);
                sum += nums[end];
                max = max.max(sum);
                end += 1;
            }
            sum -= n;
            numbers.remove(&n);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,2,4,5,6] => 17; "example 1")]
    #[test_case(vec![5,2,1,2,5,2,1,2,5] => 8; "example 2")]
    #[test_case(vec![187,470,25,436,538,809,441,167,477,110,275,133,666,345,411,459,490,266,987,965,429,166,809,340,467,318,125,165,809,610,31,585,970,306,42,189,169,743,78,810,70,382,367,490,787,670,476,278,775,673,299,19,893,817,971,458,409,886,434] => 16911; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::maximum_unique_subarray(nums)
    }
}
