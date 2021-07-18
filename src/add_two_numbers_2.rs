use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }
        let mut node1 = l1;
        let mut node2 = l2;
        let mut out = None;
        let mut carry = 0;
        while node1.is_some() || node2.is_some() || carry > 0 {
            let sum = Self::node_number(&node1) + Self::node_number(&node2) + carry;
            let n = sum % 10;
            out = Some(Box::new(ListNode { val: n, next: out }));
            carry = sum / 10;
            node1 = node1.and_then(|n| n.next);
            node2 = node2.and_then(|n| n.next);
        }
        Self::reverse_list(out)
    }
    fn node_number(node: &Option<Box<ListNode>>) -> i32 {
        if let Some(node) = node {
            node.val
        } else {
            0
        }
    }
    fn reverse_list(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = None;
        let mut node = list;
        while let Some(current) = node {
            reversed = Some(Box::new(ListNode {
                val: current.val,
                next: reversed,
            }));
            node = current.next;
        }
        reversed
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{list_to_vec, vec_to_list};

    use super::*;
    #[test]
    fn example_1() {
        let l1 = vec_to_list(vec![2, 4, 3]);
        let l2 = vec_to_list(vec![5, 6, 4]);
        assert_eq!(
            vec![7, 0, 8],
            list_to_vec(Solution::add_two_numbers(l1, l2)),
        )
    }

    #[test]
    fn case_1() {
        let l1 = vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = vec_to_list(vec![9, 9, 9, 9]);
        assert_eq!(
            vec![8, 9, 9, 9, 0, 0, 0, 1],
            list_to_vec(Solution::add_two_numbers(l1, l2)),
        )
    }
}
