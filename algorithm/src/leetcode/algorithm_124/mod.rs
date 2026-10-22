pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;
    use test_case::test_case;

    #[test_case(TreeNode::from_preorder_str(vec!["1", "2", "3"]) => 6; "example 1")]
    #[test_case(TreeNode::from_preorder_str(vec!["-10","9","20","null","null","15","7"]) => 42; "example 2")]
    #[test_case(TreeNode::from_preorder_str(vec!["-3"]) => -3; "case 1")]
    #[test_case(TreeNode::from_preorder_str(vec!["1", "2"]) => 3; "case 2")]
    #[test_case(TreeNode::from_preorder_str(vec!["1", "-2", "-3", "1", "3", "-2", "null", "-1"]) => 3; "case 3")]
    #[test_case(TreeNode::from_preorder_str(vec!["2", "-1", "-2"]) => 2; "case 4")]
    #[test_case(TreeNode::from_preorder_str(vec!["5", "4", "8", "11", "null", "13", "4", "7", "2", "null", "null", "null", "1"]) => 48; "case 5")]
    fn test_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        recursive::Solution::max_path_sum(root)
    }
}
