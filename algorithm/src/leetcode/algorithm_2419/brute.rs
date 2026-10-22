pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().cloned().unwrap();
        let mut max_length = 0;
        for i in 0..nums.len() {
            let mut end = i;
            while end < nums.len() && nums[end] ^ max == 0 {
                end += 1;
            }
            max_length = max_length.max((end - i) as i32);
        }
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,3,2,2] => 2; "example 1")]
    #[test_case(vec![1,2,3,4] => 1; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::longest_subarray(nums)
    }
}
