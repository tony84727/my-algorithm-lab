pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        for start in 0..nums.len() {
            let mut product = 1;
            for n in nums.iter().skip(start) {
                product *= n;
                max = max.max(product);
            }
        }
        max
    }
}
