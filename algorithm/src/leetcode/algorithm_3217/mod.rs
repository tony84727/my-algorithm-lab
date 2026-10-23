use std::collections::HashSet;

use crate::leetcode::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let to_remove: HashSet<i32> = HashSet::from_iter(nums);
        let mut dummy = Box::new(ListNode {
            val: 0,
            next: head.take(),
        });
        let mut current = &mut dummy;

        while current.next.is_some() {
            if to_remove.contains(&current.next.as_ref().unwrap().val) {
                let mut removed = current.next.take().unwrap();
                current.next = removed.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::common::{list_to_vec, vec_to_list};

    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3], vec![1,2,3,4,5] => vec![4,5]; "example 1")]
    #[test_case(vec![5], vec![1,2,3,4] => vec![1,2,3,4]; "example 2")]
    fn test_solution(nums: Vec<i32>, list: Vec<i32>) -> Vec<i32> {
        list_to_vec(Solution::modified_list(nums, vec_to_list(list)))
    }
}
