pub mod infix;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::TreeNode;
    use test_case::test_case;

    fn parse_input(preorder: Vec<&str>) -> Vec<Option<i32>> {
        preorder
            .into_iter()
            .map(|input| {
                if input == "null" {
                    return None;
                }
                Some(input.parse().unwrap())
            })
            .collect()
    }

    #[test_case(vec!["2", "1", "3"] => true; "example 1")]
    #[test_case(vec!["5","1", "4", "null", "null", "3", "6"] => false; "example 2")]
    #[test_case(vec!["5","4", "6", "null", "null", "3", "7"] => false; "case 1")]
    #[test_case(vec!["2", "2", "2"] => false; "case 2")]
    #[test_case(vec!["3", "1", "5", "0", "2", "4", "6"] => true; "case 3")]
    #[test_case(vec!["22", "12", "null", "-88", "null", "null", "-69"] => true; "case 4")]
    #[test_case(vec!["32", "26", "47", "19", "null", "null", "56", "null", "27"] => false; "case 5")]
    fn test_brute_solution(preorder: Vec<&str>) -> bool {
        let root = TreeNode::from_preorder(parse_input(preorder));
        infix::Solution::is_valid_bst(root)
    }
}
