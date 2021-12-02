pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn max_rob(nums: &[i32], i: usize) -> i32 {
            if i >= nums.len() {
                return 0;
            }
            let rob_current = nums[i] + max_rob(nums, i + 2);
            let rob_adjacent = max_rob(nums, i + 1);
            if rob_current > rob_adjacent {
                rob_current
            } else {
                rob_adjacent
            }
        }
        max_rob(&nums, 0)
    }
}
