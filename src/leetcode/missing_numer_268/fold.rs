pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.into_iter()
            .enumerate()
            .fold(0, |acc, (i, n)| acc ^ (i as i32) ^ n)
            ^ (len as i32)
    }
}
