pub struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        let mut mid;
        loop {
            mid = (right + left) / 2;
            let current = nums[mid];
            match current {
                x if mid > 0 && x == nums[mid - 1] => {
                    mid -= 1;
                }
                x if mid + 1 < nums.len() && x == nums[mid + 1] => (),
                _ => return current,
            };
            let left_odd = (mid - left) % 2 != 0;
            if left_odd {
                right = mid - 1;
            } else {
                left = mid + 2;
            }
        }
    }
}
