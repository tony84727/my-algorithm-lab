pub mod compare;
pub mod dump;
pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::leetcode::common::TreeNode;
    use test_case::test_case;

    #[test_case(vec!["1", "2", "3"], vec!["1", "2", "3"] => true; "example 1")]
    #[test_case(vec!["1", "2"], vec!["1", "null", "2"] => false; "case 1")]
    #[test_case(vec!["1", "2", "1"], vec!["1", "1", "2"] => false; "case 2")]
    #[test_case(vec!["0"], vec!["1"] => false; "case 3")]
    #[test_case(vec!["1", "1"], vec!["1", "null", "1"] => false; "case 4")]
    fn test_dump_solution(p: Vec<&str>, q: Vec<&str>) -> bool {
        let p = TreeNode::from_preorder_str(p);
        let q = TreeNode::from_preorder_str(q);
        dump::Solution::is_same_tree(p, q)
    }
    #[test_case(vec!["1", "2", "3"], vec!["1", "2", "3"] => true; "example 1")]
    #[test_case(vec!["1", "2"], vec!["1", "null", "2"] => false; "case 1")]
    #[test_case(vec!["1", "2", "1"], vec!["1", "1", "2"] => false; "case 2")]
    #[test_case(vec!["0"], vec!["1"] => false; "case 3")]
    #[test_case(vec!["1", "1"], vec!["1", "null", "1"] => false; "case 4")]
    fn test_compare_solution(p: Vec<&str>, q: Vec<&str>) -> bool {
        let p = TreeNode::from_preorder_str(p);
        let q = TreeNode::from_preorder_str(q);
        compare::Solution::is_same_tree(p, q)
    }

    #[test_case(vec!["1", "2", "3"], vec!["1", "2", "3"] => true; "example 1")]
    #[test_case(vec!["1", "2"], vec!["1", "null", "2"] => false; "case 1")]
    #[test_case(vec!["1", "2", "1"], vec!["1", "1", "2"] => false; "case 2")]
    #[test_case(vec!["0"], vec!["1"] => false; "case 3")]
    #[test_case(vec!["1", "1"], vec!["1", "null", "1"] => false; "case 4")]
    fn test_recursive_solution(p: Vec<&str>, q: Vec<&str>) -> bool {
        let p = TreeNode::from_preorder_str(p);
        let q = TreeNode::from_preorder_str(q);
        recursive::Solution::is_same_tree(p, q)
    }
}
