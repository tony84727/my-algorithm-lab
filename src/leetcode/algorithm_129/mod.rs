pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use super::common::TreeNode;
impl Solution {
    pub fn dfs_collect(root: Option<Rc<RefCell<TreeNode>>>, mut prefix: Vec<i32>) -> Vec<Vec<i32>> {
        match root {
            Some(root) => {
                prefix.push(root.borrow().val);
                if root.borrow().right.is_none() && root.borrow().left.is_none() {
                    return vec![prefix];
                }
                let mut left = Self::dfs_collect(root.borrow().left.clone(), prefix.clone());
                let mut right = Self::dfs_collect(root.borrow().right.clone(), prefix);
                left.append(&mut right);
                left
            }
            None => vec![],
        }
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let numbers = Self::dfs_collect(root, vec![]);
        numbers
            .into_iter()
            .flat_map(|digits| {
                digits
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(|(i, n)| n * 10_i32.pow(i as u32))
            })
            .sum::<i32>()
    }
}
