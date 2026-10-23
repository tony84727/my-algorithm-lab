pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut min_negative = nums[0];
        let mut max_positive = nums[0];
        let mut max = nums[0];
        for n in nums.into_iter().skip(1) {
            if n < 0 {
                let temp_max = max_positive;
                max_positive = n.max(min_negative * n);
                min_negative = n.min(temp_max * n);
            } else {
                min_negative = n.min(min_negative * n);
                max_positive = n.max(max_positive * n);
            }
            max = max.max(max_positive);
        }
        max
    }
}
