pub struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums
    }
}
