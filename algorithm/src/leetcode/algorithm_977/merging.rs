pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut results = vec![0_i32; nums.len()];
        let mut left = 0;
        let mut right = nums.len();
        for i in (0..nums.len()).rev() {
            let left_number = nums[left].abs();
            let right_number = nums[right - 1].abs();
            results[i] = if left_number > right_number {
                left += 1;
                left_number * left_number
            } else {
                right -= 1;
                right_number * right_number
            };
        }
        results
    }
}
