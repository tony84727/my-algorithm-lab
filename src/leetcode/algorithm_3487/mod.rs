use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut elements = HashSet::new();
        let mut length = 0;
        let mut sum = 0;
        let mut max_non_positive = i32::MIN;
        for n in nums.into_iter() {
            if n > 0 {
                if !elements.contains(&n) {
                    sum += n;
                    length += 1;
                    elements.insert(n);
                }
                continue;
            }
            max_non_positive = max_non_positive.max(n);
        }
        if length == 0 {
            return max_non_positive;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4,5] => 15; "example 1")]
    #[test_case(vec![1,1,0,1,1] => 1; "example 2")]
    #[test_case(vec![1,2,-1,-2,1,0,-1] => 3; "example 3")]
    #[test_case(vec![-100] => -100; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::max_sum(nums)
    }
}
