use crate::leetcode::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut all = lists
            .into_iter()
            .flat_map(|list| {
                let mut elements = vec![];
                let mut current = &list;
                while let Some(node) = current {
                    elements.push(node.val);
                    current = &node.next;
                }
                elements
            })
            .collect::<Vec<i32>>();
        all.sort_unstable();
        let mut out = None;
        for n in all.into_iter().rev() {
            out = Some(Box::new(ListNode { val: n, next: out }))
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::{list_to_vec, vec_to_list};
    use test_case::test_case;

    #[test_case(vec![vec![1,4,5], vec![1,3,4], vec![2,6]] => vec![1,1,2,3,4,4,5,6]; "example 1")]
    fn test_solution(list: Vec<Vec<i32>>) -> Vec<i32> {
        list_to_vec(Solution::merge_k_lists(
            list.into_iter().map(vec_to_list).collect(),
        ))
    }
}
