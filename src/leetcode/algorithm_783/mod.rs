use std::{cell::RefCell, rc::Rc};

use super::common::TreeNode;

pub struct Solution;

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        let start = root.unwrap();
        let value = start.borrow().val;
        let mut right_min = start
            .borrow()
            .right
            .as_ref()
            .map(|n| n.borrow().val)
            .unwrap_or(value);
        let mut left_max = start
            .borrow()
            .left
            .as_ref()
            .map(|n| n.borrow().val)
            .unwrap_or(value);
        let mut to_visit = vec![start];
        while let Some(current) = to_visit.pop() {
            let value = current.borrow().val;
            if let Some(left) = &current.borrow().left {
                left_max = left_max.max(left.borrow().val);
                min = min.min(right_min - left_max);
                to_visit.push(left.clone());
            }
            if let Some(right) = &current.borrow().right {
                right_min = right_min.min(right.borrow().val);
                min = min.min(right_min - left_max);
                to_visit.push(right.clone());
            }
        }
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
