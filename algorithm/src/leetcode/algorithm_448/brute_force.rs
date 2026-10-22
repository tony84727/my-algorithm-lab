pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut exists = HashSet::new();
        let max = nums.len();
        for n in nums.into_iter() {
            exists.insert(n);
        }
        let mut missing = vec![];
        for i in 1..=max as i32 {
            if !exists.contains(&i) {
                missing.push(i);
            }
        }
        missing
    }
}
