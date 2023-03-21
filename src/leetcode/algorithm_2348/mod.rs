pub struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut current = 0;
        for n in nums.into_iter() {
            if n == 0 {
                current += 1;
                count += current;
            } else {
                current = 0;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,3,0,0,2,0,0,4] => 6; "example 1")]
    #[test_case(vec![0,0,0,2,0,0] => 9; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i64 {
        Solution::zero_filled_subarray(nums)
    }
}
