use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        for i in 0..n {
            let mut seen = HashSet::new();
            for x in nums.iter().skip(i) {
                seen.insert(x);
                sum += (seen.len() as i32).pow(2);
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 1] => 15; "example 1")]
    #[test_case(vec![1, 1] => 3; "example 2")]
    fn test_sum_counts(nums: Vec<i32>) -> i32 {
        Solution::sum_counts(nums)
    }
}
