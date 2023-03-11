use std::{cell::RefCell, rc::Rc};

use crate::leetcode::common::{ListNode, TreeNode};

pub struct Solution;

impl Solution {
    fn insert(left: &[i32], val: i32, right: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let left = match left.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(left[0])))),
            len => {
                let mid = len / 2;
                Self::insert(&left[..mid], left[mid], &left[mid + 1..])
            }
        };
        let right = match right.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(right[0])))),
            len => {
                let mid = len / 2;
                Self::insert(&right[..mid], right[mid], &right[mid + 1..])
            }
        };
        let current = Rc::new(RefCell::new(TreeNode::new(val)));
        current.borrow_mut().right = right;
        current.borrow_mut().left = left;
        Some(current)
    }
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut all = vec![];
        let mut current = &head;
        while let Some(node) = current {
            all.push(node.val);
            current = &node.next;
        }
        if all.is_empty() {
            return None;
        }
        if all.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(all[0]))));
        }
        let mid = all.len() / 2;
        Self::insert(&all[..mid], all[mid], &all[mid + 1..])
    }
}
