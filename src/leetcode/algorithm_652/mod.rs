use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::common::TreeNode;
pub struct Solution;

impl Solution {
    pub fn dfs(
        subtrees: &mut HashMap<Vec<Option<i32>>, i32>,
        duplicated: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<i32>> {
        match root {
            Some(root) => {
                let mut sequence = vec![Some(root.borrow().val)];
                sequence.append(&mut Self::dfs(
                    subtrees,
                    duplicated,
                    root.borrow().right.clone(),
                ));
                sequence.append(&mut Self::dfs(
                    subtrees,
                    duplicated,
                    root.borrow().left.clone(),
                ));
                let entry = subtrees.entry(sequence.clone()).or_default();
                if *entry == 1 {
                    duplicated.push(Some(root));
                }
                *entry += 1;
                sequence
            }
            None => vec![None],
        }
    }
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut duplicated = vec![];
        let mut subtrees = HashMap::new();
        Self::dfs(&mut subtrees, &mut duplicated, root);
        duplicated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::TreeNode;
    use test_case::test_case;

    #[test_case(vec!["1", "2", "3", "4", "null", "2", "4", "null", "null", "4"] => vec![vec![2, 4], vec![4]]; "example 1")]
    #[test_case(vec!["2", "1", "1"] => vec![vec![1]]; "example 2")]
    #[test_case(vec!["2", "2", "2", "3", "null", "3", "null"] => vec![vec![2, 3], vec![3]]; "example 3")]
    fn test_solution(preorder: Vec<&str>) -> Vec<Vec<i32>> {
        let root = TreeNode::from_preorder_str(preorder);
        let mut values = Solution::find_duplicate_subtrees(root)
            .into_iter()
            .map(TreeNode::serialize)
            .collect::<Vec<_>>();
        values.sort();
        values
    }
}
