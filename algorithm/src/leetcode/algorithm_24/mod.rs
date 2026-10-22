use crate::leetcode::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut numbers = vec![];
        let mut current = head;
        while let Some(node) = current {
            numbers.push(node.val);
            current = node.next;
        }
        if numbers.is_empty() {
            return None;
        }
        Self::swap(&mut numbers);
        let mut out = None;
        for n in numbers.into_iter().rev() {
            out = Some(Box::new(ListNode { val: n, next: out }))
        }
        out
    }

    fn swap(values: &mut [i32]) {
        for i in (0..values.len()).skip(1).filter(|x| x % 2 != 0) {
            values.swap(i - 1, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::common::{list_to_vec, vec_to_list};

    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4] => vec![2,1,4,3]; "example 1")]
    #[test_case(vec![] => Vec::<i32>::new(); "example 2")]
    #[test_case(vec![1] => vec![1]; "example 3")]
    fn test_solution(list: Vec<i32>) -> Vec<i32> {
        list_to_vec(Solution::swap_pairs(vec_to_list(list)))
    }
}
