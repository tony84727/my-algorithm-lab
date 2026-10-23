use crate::leetcode::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dump(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
            let mut stack = VecDeque::new();
            stack.push_front(root);
            let mut collected = vec![];
            while !stack.is_empty() {
                let current = stack.pop_front().unwrap();
                collected.push(current.as_ref().map(|node| node.borrow().val));
                if let Some(current) = current {
                    stack.push_front(current.borrow().right.clone());
                    stack.push_front(current.borrow().left.clone());
                }
            }
            collected
        }
        dump(p) == dump(q)
    }
}
