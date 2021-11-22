// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (last, current) = {
            let mut last = None;
            let mut current = root;
            while let Some(node) = &current {
                last = Some(node);
                match node {
                    x if key < x.borrow().val => {
                        current = x.clone().borrow().left;
                    }
                    x if key > x.borrow().val => {
                        current = x.clone().borrow().right;
                    }
                };
            }
            (last, current)
        };
        root
    }
}
