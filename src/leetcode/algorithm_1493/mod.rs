pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut right = 0;
        let mut one_zero = false;
        let mut length = 0;
        for (left, &out) in nums.iter().enumerate() {
            while right < nums.len() && (!one_zero || nums[right] == 1) {
                if nums[right] == 0 {
                    one_zero = true;
                }
                right += 1;
            }
            length = length.max(right - left - 1);
            if out == 0 {
                one_zero = false;
            }
        }
        length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,0,1] => 3; "example 1")]
    #[test_case(vec![0,1,1,1,0,1,1,0,1] => 5; "example 2")]
    #[test_case(vec![1,1,1] => 2; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::longest_subarray(nums)
    }
}
