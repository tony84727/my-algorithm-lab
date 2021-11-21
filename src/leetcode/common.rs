use std::{cell::RefCell, rc::Rc};
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

    #[cfg(test)]
    /// Leetcode way to traverse a tree and gather the elements into a Vec. -1 is used as a sentinel value to avoid the need to use Option to represent
    /// a missing node
    pub fn traverse_natural_number(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        TreeNode::traverse(root)
            .into_iter()
            .map(|element| match element {
                Some(element) => element,
                None => -1,
            })
            .collect()
    }

    #[cfg(test)]
    pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        use std::{collections::VecDeque, ops::Deref};

        let mut elements = vec![];
        let mut worklist = VecDeque::new();
        if let Some(node) = root.clone() {
            elements.push(Some(node.borrow().val));
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
                    elements.push(Some(left.borrow().val));
                    worklist.push_back(Some(left.clone()));
                }
                None => {
                    elements.push(None);
                }
            }
            match &node.right {
                Some(right) => {
                    elements.push(Some(right.borrow().val));
                    worklist.push_back(Some(right.clone()));
                }
                None => {
                    elements.push(None);
                }
            }
        }
        elements
    }
}
