pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (0..=nums.len()).fold(nums.into_iter().fold(0, |acc, n| acc ^ n), |acc, n| {
            acc ^ (n as i32)
        })
    }
}
