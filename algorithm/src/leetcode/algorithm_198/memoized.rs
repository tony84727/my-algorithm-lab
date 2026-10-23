use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut solutions = HashMap::new();
        fn memoized_max_rob(solutions: &mut HashMap<usize, i32>, nums: &[i32], i: usize) -> i32 {
            if i >= nums.len() {
                return 0;
            }
            if let Some(&answer) = solutions.get(&i) {
                return answer;
            }
            let rob_current = nums[i] + memoized_max_rob(solutions, nums, i + 2);
            let rob_adjacent = memoized_max_rob(solutions, nums, i + 1);
            let answer = if rob_current > rob_adjacent {
                rob_current
            } else {
                rob_adjacent
            };
            solutions.insert(i, answer);
            answer
        }
        memoized_max_rob(&mut solutions, &nums, 0)
    }
}
