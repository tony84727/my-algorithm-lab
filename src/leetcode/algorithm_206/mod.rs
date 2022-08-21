use crate::leetcode::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut next = head;
        let mut reversed: Option<Box<ListNode>> = None;
        while let Some(next_node) = next {
            let mut current = next_node;
            next = current.next;
            current.next = reversed;
            reversed = Some(current);
        }
        reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::{list_to_vec, vec_to_list};
    use quickcheck::quickcheck;

    quickcheck! {
        fn quickcheck_reverse(list: Vec<i32>) -> bool {
            let expected = {
                let mut v = list.clone();
                v.reverse();
                v
            };
            let node = vec_to_list(list);
            expected == list_to_vec(Solution::reverse_list(node))
        }
    }
}
