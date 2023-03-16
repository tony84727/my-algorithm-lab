pub struct Solution;

use crate::leetcode::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct DfsIterator {
    to_visit: Vec<Option<(Rc<RefCell<TreeNode>>, bool)>>,
    reverse: bool,
}

impl DfsIterator {
    fn new(start: Option<Rc<RefCell<TreeNode>>>, reverse: bool) -> Self {
        let to_visit = vec![start.map(|node| (node, false))];
        Self { reverse, to_visit }
    }
}

impl Iterator for DfsIterator {
    type Item = Option<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.to_visit.pop() {
            match current {
                Some((current, false)) => {
                    self.to_visit.push(Some((current.clone(), true)));
                    let mut current = Some(current);
                    while let Some(node) = current {
                        let current_borrowed = node.borrow();
                        if self.reverse {
                            self.to_visit
                                .push(current_borrowed.left.clone().map(|node| (node, false)));
                            self.to_visit
                                .push(current_borrowed.right.clone().map(|node| (node, false)));
                        } else {
                            self.to_visit
                                .push(current_borrowed.right.clone().map(|node| (node, false)));
                            self.to_visit
                                .push(current_borrowed.left.clone().map(|node| (node, false)));
                        }
                        current = if self.reverse {
                            current_borrowed.right.clone()
                        } else {
                            current_borrowed.left.clone()
                        };
                    }
                }
                Some((current, true)) => {
                    return Some(Some(current.borrow().val));
                }
                None => {
                    return Some(None);
                }
            }
        }
        None
    }
}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => {
                let root = root.borrow();
                let mut left = DfsIterator::new(root.left.clone(), false);
                let mut right = DfsIterator::new(root.right.clone(), true);
                loop {
                    match (left.next(), right.next()) {
                        (Some(left), Some(right)) if left != right => return false,
                        (Some(_), None) | (None, Some(_)) => return false,
                        (None, None) => break,
                        _ => (),
                    }
                }
                true
            }
            None => false,
        }
    }
}
