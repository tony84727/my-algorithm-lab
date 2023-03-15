use crate::leetcode::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut level = vec![root];
        while !level.is_empty() {
            let current = std::mem::take(&mut level);
            let mut end = false;
            for node in current.into_iter() {
                match node {
                    Some(node) => {
                        if end {
                            return false;
                        }
                        let node = node.borrow();
                        level.push(node.left.clone());
                        level.push(node.right.clone());
                    }
                    None => {
                        end = true;
                    }
                }
            }
            if end && level.iter().any(|x| x.is_some()) {
                return false;
            }
        }
        true
    }
}
