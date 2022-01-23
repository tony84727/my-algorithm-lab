pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last: i32 = 0;
        let max = nums.len() - 1;
        while (last as usize) < max {
            let origin = last;
            let mut j = last;
            while j <= last && j <= (max as i32) {
                last = last.max(j + nums[j as usize]);
                j += 1;
            }
            if origin == last {
                return false;
            }
        }
        true
    }
}
