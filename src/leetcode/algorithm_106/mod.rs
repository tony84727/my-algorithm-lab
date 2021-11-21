pub struct Solution;

use super::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if postorder.is_empty() {
                return None;
            }
            if postorder.len() <= 1 {
                return Some(Rc::new(RefCell::new(TreeNode::new(postorder[0]))));
            }
            let &root_value = postorder.last().unwrap();
            let postorder = &postorder[..postorder.len() - 1];
            let mut root = TreeNode::new(root_value);
            let root_inorder_index = inorder.iter().position(|&x| x == root_value).unwrap();
            let left_inorders = &inorder[..root_inorder_index];
            let left_postorder: &[i32] = postorder
                .iter()
                .enumerate()
                .rev()
                .find(|(_, x)| left_inorders.contains(x))
                .map(|(i, _)| i)
                .map_or(&[], |i| &postorder[..=i]);
            root.left = build(left_inorders, left_postorder);
            let right_inorders = &inorder[root_inorder_index + 1..];
            let right_postorder: &[i32] = postorder
                .iter()
                .enumerate()
                .rev()
                .find(|(_, x)| right_inorders.contains(x))
                .map(|(i, _)| i)
                .map_or(&[], |i| &postorder[..=i]);
            root.right = build(right_inorders, right_postorder);
            Some(Rc::new(RefCell::new(root)))
        }
        build(&inorder, &postorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ron::de::from_reader;
    use serde::Deserialize;
    use std::fs::File;
    use test_case::test_case;

    #[derive(Deserialize, Debug)]
    struct TestCase {
        inorder: Vec<i32>,
        postorder: Vec<i32>,
        expected: Vec<Option<i32>>,
    }

    #[test_case("src/leetcode/algorithm_106/example_1.ron"; "example 1")]
    #[test_case("src/leetcode/algorithm_106/case_1.ron"; "case 1")]
    fn test_solution(case_file_path: &str) {
        let spec = File::open(case_file_path).expect("fail to load test case sepc file");
        let TestCase {
            inorder,
            postorder,
            expected,
        } = from_reader(spec).expect("fail to parse the test case spec file");
        assert_eq!(
            expected,
            TreeNode::traverse(Solution::build_tree(inorder, postorder))
        )
    }
}
