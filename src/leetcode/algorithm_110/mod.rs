use std::{cell::RefCell, rc::Rc};

use super::common::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let right = Self::get_height(node.right.clone());
                let left = Self::get_height(node.left.clone());
                (right - left).abs() <= 1
                    && Self::is_balanced(node.right.clone())
                    && Self::is_balanced(node.left.clone())
            }
        }
    }

    fn get_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let node = root.borrow();
                1 + Self::get_height(node.left.clone()).max(Self::get_height(node.right.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::TreeNode;
    use test_case::test_case;

    #[test_case(TreeNode::from_preorder_str(vec!["3", "9", "20", "null", "null", "15", "7"]), true; "example 1")]
    #[test_case(TreeNode::from_preorder_str(vec!["1", "2", "2", "3", "3", "null", "null", "4", "4"]), false; "example 2")]
    #[test_case(None, true; "example 3")]
    #[test_case(TreeNode::from_preorder_str(vec!["1", "null", "2", "null", "3"]), false; "case 1")]
    #[test_case(TreeNode::from_preorder_str(vec!["1", "2", "2", "3", "null", "null", "3", "4", "null", "null", "4"]), false; "case 2")]
    fn test_solution(root: Option<Rc<RefCell<TreeNode>>>, expected: bool) {
        assert_eq!(expected, Solution::is_balanced(root));
    }
}
