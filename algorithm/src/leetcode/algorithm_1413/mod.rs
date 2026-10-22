pub struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut max = 1;
        let mut sum = 1;
        for n in nums.into_iter() {
            sum -= n;
            if sum > max {
                max = sum;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-3,2,-3,4,2] => 5; "example 1")]
    #[test_case(vec![1,2] => 1; "example 2")]
    #[test_case(vec![1,-2,-3] => 5; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::min_start_value(nums)
    }
}
