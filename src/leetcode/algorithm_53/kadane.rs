pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut max_ending = nums[0];
        for x in nums.into_iter().skip(1) {
            max_ending = x.max(max_ending + x);
            max_sum = max_sum.max(max_ending);
        }
        max_sum
    }
}
