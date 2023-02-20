pub struct Solution;

use crate::leetcode::common::TreeNode;
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        match root {
            None => vec![],
            Some(root) => {
                let mut levels = vec![];
                let mut current_level = vec![];
                let mut to_visit = vec![root];
                let mut next_level_to_visit = vec![];
                let mut reverse = false;
                while !to_visit.is_empty() {
                    for current in to_visit.iter() {
                        let current = current.borrow();
                        current_level.push(current.val);
                        if let Some(left) = current.left.clone() {
                            next_level_to_visit.push(left);
                        }
                        if let Some(right) = current.right.clone() {
                            next_level_to_visit.push(right);
                        }
                    }
                    to_visit = std::mem::replace(&mut next_level_to_visit, vec![]);
                    if reverse {
                        current_level.reverse();
                    }
                    levels.push(std::mem::replace(&mut current_level, vec![]));
                    reverse = !reverse;
                }
                levels
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["3", "9", "20", "null", "null", "15", "7"] => vec![vec![3], vec![20,9], vec![15,7]]; "example 1")]
    fn test_solution(tree: &[&str]) -> Vec<Vec<i32>> {
        Solution::zigzag_level_order(TreeNode::from_preorder_str(tree.to_vec()))
    }
}
