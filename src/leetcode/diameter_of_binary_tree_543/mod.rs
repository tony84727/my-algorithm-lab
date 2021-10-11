use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};
pub struct Solution;

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

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let mut count = 0;
                let mut stack = VecDeque::new();
                stack.push_back(root);
                while !stack.is_empty() {
                    count += 1;
                    let visiting = stack.pop_back().unwrap();
                    if let Some(right) = visiting.deref().borrow().right.clone() {
                        stack.push_back(right);
                    }
                    if let Some(left) = visiting.deref().borrow().left.clone() {
                        stack.push_back(left);
                    };
                }
                return ((count + 1) as f32).log2().floor() as i32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn build_binary_tree(elements: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tree = None;
        if elements.is_empty() {
            return tree;
        }
        'insert: for n in elements.iter() {
            if tree.is_none() {
                tree = Some(Rc::new(RefCell::new(TreeNode::new(*n))));
                continue;
            }
            let mut queue = vec![tree.as_ref().unwrap().clone()];
            while !queue.is_empty() {
                let current = queue.pop().unwrap();
                match &mut current.deref().borrow_mut().left {
                    Some(left) => {
                        queue.push(left.clone());
                    }
                    left @ None => {
                        *left = Some(Rc::new(RefCell::new(TreeNode::new(*n))));
                        continue 'insert;
                    }
                }
                match &mut current.deref().borrow_mut().right {
                    Some(right) => {
                        queue.push(right.clone());
                    }
                    right @ None => {
                        *right = Some(Rc::new(RefCell::new(TreeNode::new(*n))));
                        continue 'insert;
                    }
                };
            }
        }
        tree
    }

    #[test_case(&[1,2,3,4,5] => 3; "example 1")]
    #[test_case(&[1,2] => 1; "case 1")]
    #[test_case(&[1] => 1; "case 2")]
    fn test_solution(elements: &[i32]) -> i32 {
        Solution::diameter_of_binary_tree(build_binary_tree(elements))
    }
}
