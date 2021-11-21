use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

#[cfg(test)]
pub fn vec_to_list(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut out = None;
    for n in numbers.into_iter().rev() {
        out = Some(Box::new(ListNode { val: n, next: out }))
    }
    out
}

#[cfg(test)]
pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![];
    let mut node = list;
    while let Some(current) = node {
        out.push(current.val);
        node = current.next;
    }
    out
}

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

    pub fn traverse_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements = vec![];
        let mut worklist = VecDeque::new();
        if let Some(node) = root.clone() {
            elements.push(node.borrow().val);
            worklist.push_back(root);
        }
        while !worklist.is_empty() {
            let node = worklist.pop_front().unwrap();
            let node = node.unwrap();
            let node = node.deref().borrow();
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
}
