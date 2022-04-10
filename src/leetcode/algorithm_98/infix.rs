use crate::leetcode::common::TreeNode;
use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};
pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = VecDeque::new();
        let mut min = None;
        let mut current = root;
        while !stack.is_empty() || current.is_some() {
            while let Some(now) = current.clone() {
                stack.push_front(now.clone());
                current = now.deref().borrow().left.clone();
            }
            let now = stack.pop_front().unwrap();
            let val = now.deref().borrow().val;
            if let Some(m) = min {
                if val <= m {
                    return false;
                }
            }
            min = Some(val);
            current = now.deref().borrow().right.clone();
        }
        true
    }
}
