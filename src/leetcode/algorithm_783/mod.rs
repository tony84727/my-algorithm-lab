use std::{cell::RefCell, rc::Rc};

use super::common::TreeNode;

pub struct Solution;

impl Solution {
    fn travel(last: &mut Option<i32>, min: &mut i32, node: Rc<RefCell<TreeNode>>) {
        let node = node.borrow();
        if let Some(left) = &node.left {
            Self::travel(last, min, left.clone());
        }
        let value = node.val;
        if let Some(last) = last {
            *min = (*min).min(value - *last);
        }
        *last = Some(value);
        if let Some(right) = &node.right {
            Self::travel(last, min, right.clone());
        }
    }
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        let mut last = None;
        Self::travel(&mut last, &mut min, root.unwrap());
        min
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::common::TreeNode;

    use super::*;
    use test_case::test_case;

    #[test_case(vec!["4","2","6","1","3"] => 1; "example 1")]
    #[test_case(vec!["90","69","null","49","89","null","52"] => 1; "case 1")]
    fn test_solution(values: Vec<&str>) -> i32 {
        Solution::min_diff_in_bst(TreeNode::from_preorder_str(values))
    }
}
