use crate::leetcode::common::ListNode;
pub struct Solution;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut items = vec![];
        let mut current = head;
        while let Some(node) = current {
            items.push(node.val);
            current = node.next;
        }
        let mut max = 0;
        for (i, a) in items.iter().enumerate().take(items.len() / 2) {
            max = max.max(a + items[items.len() - i - 1]);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::common::vec_to_list;

    use super::*;
    use test_case::test_case;

    #[test_case(vec![5,4,2,1] => 6; "example 1")]
    #[test_case(vec![4,2,2,3] => 7; "example 2")]
    fn test_solution(items: Vec<i32>) -> i32 {
        Solution::pair_sum(vec_to_list(items))
    }
}
