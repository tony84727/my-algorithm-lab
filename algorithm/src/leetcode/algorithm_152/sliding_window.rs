pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut product = 1;
        let mut right = 0;
        let mut left = 0;
        while left < nums.len() {
            while right < nums.len() && nums[right] != 0 {
                product *= nums[right];
                max = max.max(product);
                right += 1;
            }
            if nums[left] == 0 {
                max = max.max(0);
                right = left + 1;
                product = 1;
            } else {
                product /= nums[left];
            }
            if left + 1 != right {
                max = max.max(product);
            }
            left += 1;
        }
        max
    }
}
