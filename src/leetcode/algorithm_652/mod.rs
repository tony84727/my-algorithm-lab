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
