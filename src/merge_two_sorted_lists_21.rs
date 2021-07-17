use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut n1 = l1;
        let mut n2 = l2;
        let mut out = None;
        while n1.is_some() || n2.is_some() {
            let number1 = n1.as_ref().map_or(101, |n| n.val);
            let number2 = n2.as_ref().map_or(101, |n| n.val);
            let next_number = {
                if number2 < number1 {
                    n2 = n2.unwrap().next;
                    number2
                } else {
                    n1 = n1.unwrap().next;
                    number1
                }
            };
            out = Some(Box::new(ListNode {
                next: out,
                val: next_number,
            }));
        }
        Self::reverse_list(out)
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
    use super::*;
    use crate::common::{list_to_vec, vec_to_list};

    #[test]
    fn test_example_1() {
        let l1 = vec_to_list(vec![1, 2, 4]);
        let l2 = vec_to_list(vec![1, 3, 4]);
        assert_eq!(
            vec![1, 1, 2, 3, 4, 4],
            list_to_vec(Solution::merge_two_lists(l1, l2))
        );
    }
}
