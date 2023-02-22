pub struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, c| acc ^ c)
    }
}
