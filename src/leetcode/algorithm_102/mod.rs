use crate::leetcode::common::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut collected = vec![];
        let mut todo = vec![root];
        while !todo.is_empty() {
            let mut this_level_values = Vec::new();
            let mut next_level = vec![];
            for current in todo.iter() {
                match current {
                    None => {
                        continue;
                    }
                    Some(node) => {
                        let node = RefCell::borrow(node);
                        this_level_values.push(node.val);
                        next_level.push(node.left.clone());
                        next_level.push(node.right.clone());
                    }
                }
            }
            if !this_level_values.is_empty() {
                collected.push(this_level_values);
            }
            todo = next_level;
        }
        collected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_macro::vecvec;
    use crate::leetcode::common::TreeNode;
    use test_case::test_case;

    #[test_case(vec!["3", "9", "20", "null", "null", "15", "7"] => vecvec![[3], [9,20], [15,7]]; "example 1")]
    fn test_solution(root: Vec<&str>) -> Vec<Vec<i32>> {
        let root = TreeNode::from_preorder_str(root);
        Solution::level_order(root)
    }
}
