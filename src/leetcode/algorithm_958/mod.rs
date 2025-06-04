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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::TreeNode;
    use test_case::test_case;

    #[test_case(vec!["1", "2", "3", "4", "5", "6"] => true; "example 1")]
    #[test_case(vec!["1", "2", "3", "4", "5", "null", "7"] => false; "example 2")]
    #[test_case(vec!["1", "2", "3", "4", "5", "6", "7"] => true; "full tree")]
    #[test_case(vec!["1", "2", "3", "4", "null", "6", "7"] => false; "missing right before left")]
    fn test_solution(preorder: Vec<&str>) -> bool {
        let root = TreeNode::from_preorder_str(preorder);
        Solution::is_complete_tree(root)
    }
}
