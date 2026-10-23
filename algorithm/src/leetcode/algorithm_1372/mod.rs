pub struct Solution;

use crate::leetcode::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let mut max = 0;
                let mut todo = vec![(0, 0, root)];
                while let Some((left, right, node)) = todo.pop() {
                    if let Some(l) = &node.borrow().left {
                        todo.push((right + 1, 0, l.clone()));
                        max = max.max(right + 1);
                    }
                    if let Some(r) = &node.borrow().right {
                        todo.push((0, left + 1, r.clone()));
                        max = max.max(left + 1);
                    }
                }
                max
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["1", "null","1","1","1","null","null","1","1","null","1","null","null","null","null","1","null","1"] => 3; "example 1")]
    #[test_case(&["1", "1", "1", "null", "1", "null", "null", "1","1", "null","1"] => 4; "example 2")]
    fn test_solution(tree: &[&'static str]) -> i32 {
        Solution::longest_zig_zag(TreeNode::from_preorder_str(tree.to_vec()))
    }
}
