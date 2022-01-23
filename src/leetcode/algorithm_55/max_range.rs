pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last: i32 = 0;
        let max = (nums.len() - 1) as i32;
        let mut j = last;
        while j <= last && j <= max {
            last = last.max(j + nums[j as usize]);
            if last >= max {
                return true;
            }
            j += 1;
        }
        false
    }
}
