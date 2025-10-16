use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut bag = BinaryHeap::new();
        for n in nums.into_iter() {
            bag.push(Reverse(n % value));
        }
        let mut current = 0;
        while let Some(Reverse(candidate)) = bag.pop() {
            if candidate < current {
                bag.push(Reverse(candidate + value));
                continue;
            }
            if candidate != current {
                break;
            }
            current += 1;
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,-10,7,13,6,8], 5 => 4; "example 1")]
    #[test_case(vec![1,-10,7,13,6,8], 7 => 2; "example 2")]
    fn test_solution(nums: Vec<i32>, value: i32) -> i32 {
        Solution::find_smallest_integer(nums, value)
    }
}
