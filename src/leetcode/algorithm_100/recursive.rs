use crate::leetcode::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.as_ref().map(|node| node.borrow().val) != q.as_ref().map(|node| node.borrow().val) {
            return false;
        }
        Self::is_same_tree(
            p.as_ref().and_then(|node| node.borrow().right.clone()),
            q.as_ref().and_then(|node| node.borrow().right.clone()),
        ) && Self::is_same_tree(
            p.as_ref().and_then(|node| node.borrow().left.clone()),
            q.as_ref().and_then(|node| node.borrow().left.clone()),
        )
    }
}
