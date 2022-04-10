use super::common::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

impl Solution {
    fn insert_bst(root: Rc<RefCell<TreeNode>>, i: i32) {
        let mut current = root;
        loop {
            if i < current.borrow().val {
                match &mut current.clone().borrow_mut().left {
                    Some(next) => {
                        current = next.clone();
                    }
                    left @ None => {
                        *left = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                        return;
                    }
                };
            } else {
                match &mut current.clone().borrow_mut().right {
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
    use std::collections::VecDeque;

    use super::*;
    use test_case::test_case;

    fn traverse_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements = vec![];
        let mut worklist = VecDeque::new();
        if let Some(node) = root.clone() {
            elements.push(node.borrow().val);
            worklist.push_back(root);
        }
        while !worklist.is_empty() {
            let node = worklist.pop_front().unwrap();
            let node = node.unwrap();
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                continue;
            }
            match &node.left {
                Some(left) => {
                    elements.push(left.borrow().val);
                    worklist.push_back(Some(left.clone()));
                }
                None => {
                    elements.push(-1);
                }
            }
            match &node.right {
                Some(right) => {
                    elements.push(right.borrow().val);
                    worklist.push_back(Some(right.clone()));
                }
                None => {
                    elements.push(-1);
                }
            }
        }
        elements
    }

    #[test_case(vec![8,5,1,7,10,12] => vec![8,5,10,1,7,-1,12]; "example 1")]
    fn test_solution(preorder: Vec<i32>) -> Vec<i32> {
        traverse_binary_tree(Solution::bst_from_preorder(preorder))
    }
}
