use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::common::TreeNode;

pub struct Solution;

impl Solution {
    fn insert_bst(root: Rc<RefCell<TreeNode>>, i: i32) {
        let mut current = root;
        loop {
            if i < current.deref().borrow().val {
                match &mut current.clone().deref().borrow_mut().left {
                    Some(next) => {
                        current = next.clone();
                    }
                    left @ None => {
                        *left = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                        return;
                    }
                };
            } else {
                match &mut current.clone().deref().borrow_mut().right {
                    Some(next) => {
                        current = next.clone();
                    }
                    right @ None => {
                        *right = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                        return;
                    }
                }
            }
        }
    }
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;
        for i in preorder.into_iter() {
            match &root {
                Some(r) => Self::insert_bst(r.clone(), i),
                None => {
                    root = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                }
            };
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![8,5,1,7,10,12] => vec![8,5,10,1,7,-1,12]; "example 1")]
    fn test_solution(preorder: Vec<i32>) -> Vec<i32> {
        TreeNode::traverse_natural_number(Solution::bst_from_preorder(preorder))
    }
}
