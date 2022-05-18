use crate::leetcode::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn find_max(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(root) = root {
                let root = root.borrow();
                let (right_connectable, right) = find_max(root.right.clone());
                let (left_connectable, left) = find_max(root.left.clone());
                let node = root.val;
                let to_parent = node.max(right_connectable + node)
                        .max(left_connectable + node);
                return (
                    to_parent,
                    right
                        .max(left)
                        .max(right_connectable + left_connectable + node)
                        .max(to_parent)
                );
            }
            (-1001, -1001)
        }
        let (_, max) = find_max(root);
        max
    }
}
