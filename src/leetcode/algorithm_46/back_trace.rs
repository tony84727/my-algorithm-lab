use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answers = vec![];
        let mut stack: VecDeque<(Vec<i32>, Vec<i32>)> = VecDeque::new();
        stack.push_front((nums, vec![]));
        while !stack.is_empty() {
            let (candidates, combination) = stack.pop_front().unwrap();
            if candidates.is_empty() {
                answers.push(combination);
                continue;
            }
            for (i, &c) in candidates.iter().enumerate() {
                let mut candidates = candidates.clone();
                candidates.remove(i);
                let mut new_combination = combination.clone();
                new_combination.push(c);
                stack.push_front((candidates, new_combination));
            }
        }
        answers
    }
}
